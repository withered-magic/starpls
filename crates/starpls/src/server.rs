use std::fs;
use std::mem;
use std::panic;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use log::debug;
use log::error;
use log::info;
use lsp_server::Connection;
use lsp_server::ReqQueue;
use parking_lot::RwLock;
use rustc_hash::FxHashSet;
use starpls_bazel::build_language::decode_rules;
use starpls_bazel::client::BazelCLI;
use starpls_bazel::client::BazelClient;
use starpls_bazel::decode_builtins;
use starpls_bazel::APIContext;
use starpls_bazel::Builtins;
use starpls_common::Dialect;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_ide::Analysis;
use starpls_ide::AnalysisSnapshot;
use starpls_ide::Change;
use starpls_ide::InferenceOptions;

use crate::config::ServerConfig;
use crate::debouncer::AnalysisDebouncer;
use crate::diagnostics::DiagnosticsManager;
use crate::document::DefaultFileLoader;
use crate::document::DocumentChangeKind;
use crate::document::DocumentManager;
use crate::document::PathInterner;
use crate::event_loop::FetchExternalReposProgress;
use crate::event_loop::RefreshAllWorkspaceTargetsProgress;
use crate::event_loop::Task;
use crate::task_pool::TaskPool;
use crate::task_pool::TaskPoolHandle;

const DEBOUNCE_INTERVAL: Duration = Duration::from_millis(250);

const BAZEL_INIT_ERR_MESSAGE: &str = "Failed to fetch Bazel configuration! Please check the language server logs for more details. Certain features may not work correctly until the underlying issue is fixed.";

pub(crate) struct Server {
    pub(crate) config: Arc<ServerConfig>,
    pub(crate) connection: Connection,
    pub(crate) req_queue: ReqQueue<(), ()>,
    pub(crate) task_pool_handle: TaskPoolHandle<Task>,
    pub(crate) document_manager: Arc<RwLock<DocumentManager>>,
    pub(crate) diagnostics_manager: DiagnosticsManager,
    pub(crate) analysis: Analysis,
    pub(crate) analysis_debouncer: AnalysisDebouncer,
    pub(crate) analysis_requested_for_files: Option<Vec<FileId>>,
    pub(crate) bazel_client: Arc<dyn BazelClient>,
    pub(crate) pending_repos: FxHashSet<String>,
    pub(crate) pending_files: FxHashSet<FileId>,
    pub(crate) force_analysis_for_files: FxHashSet<FileId>,
    pub(crate) fetched_repos: FxHashSet<String>,
    pub(crate) is_fetching_repos: bool,
    pub(crate) is_refreshing_all_workspace_targets: bool,
    pub(crate) bzlmod_enabled: bool,
}

pub(crate) struct ServerSnapshot {
    pub(crate) config: Arc<ServerConfig>,
    pub(crate) analysis_snapshot: AnalysisSnapshot,
    pub(crate) document_manager: Arc<RwLock<DocumentManager>>,
}

impl Server {
    pub(crate) fn new(connection: Connection, config: ServerConfig) -> anyhow::Result<Self> {
        // Create the task pool for processing incoming requests.
        let (task_pool_sender, task_pool_receiver) = crossbeam_channel::unbounded();
        let task_pool = TaskPool::with_num_threads(task_pool_sender.clone(), 4)?;
        let task_pool_handle = TaskPoolHandle::new(task_pool_receiver, task_pool);
        let mut has_bazel_init_err = false;

        // Load Bazel builtins from the specified file.
        let builtins = match load_bazel_builtins() {
            Ok(builtins) => builtins,
            Err(err) => {
                error!("failed to load builtins, {}", err);
                Default::default()
            }
        };

        debug!("fetching Bazel configuration");

        let bazel_path = config
            .args
            .bazel_path
            .clone()
            .unwrap_or("bazel".to_string());

        debug!("using Bazel executable at {:?}", bazel_path);

        let bazel_client = Arc::new(BazelCLI::new(&bazel_path));
        let info = match bazel_client.info() {
            Ok(info) => info,
            Err(err) => {
                error!("failed to run fetch Bazel configuration: {}", err);
                has_bazel_init_err = true;
                Default::default()
            }
        };

        info!("Bazel version: {}", info.release);
        info!("workspace root: {:?}", info.workspace);
        info!("workspace name: {:?}", info.workspace_name);

        // Determine the output base for the purpose of resolving external repositories.
        let external_output_base = info.output_base.join("external");

        info!("external output base: {:?}", external_output_base);
        info!("starlark-semantics: {:?}", info.starlark_semantics);

        // We determine whether to use bzlmod in two steps. First, we check if `MODULE.bazel` exists at all,
        // and if so, whether the `bazel mod dump_repo_mapping` command is supported. If either of these
        // checks fails, then we can't use bzlmod anyways.
        let bzlmod_capability = info
            .workspace
            .join("MODULE.bazel")
            .try_exists()
            .unwrap_or(false)
            && {
                debug!("checking for `bazel mod dump_repo_mapping` capability");
                match bazel_client.dump_repo_mapping("") {
                    Ok(_) => true,
                    Err(err) => {
                        has_bazel_init_err = true;
                        error!(
                            "failed to run `bazel mod dump_repo_mapping`, disabling bzlmod support: {}", err
                        );
                        false
                    }
                }
            };

        let bzlmod_enabled = bzlmod_capability && {
            // Next, we check if bzlmod is enabled by default for the current Bazel version.
            // bzlmod is enabled by default for Bazel versions 7 and later.
            // TODO(withered-magic): Just hardcoding this for now since I'm lazy to parse the actual versions.
            // This should last us pretty long since Bazel 9 isn't anywhere on the horizon.
            let bzlmod_enabled_by_default = ["release 7", "release 8", "release 9"]
                .iter()
                .any(|release| info.release.starts_with(release));

            if bzlmod_enabled_by_default {
                info!("Bazel 7 or later detected")
            }

            // Finally, check starlark-semantics to determine whether bzlmod has been explicitly
            // enabled/disabled, e.g. in a .bazelrc file.
            if info.starlark_semantics.contains("enable_bzlmod=true") {
                info!("found enable_bzlmod=true in starlark-semantics");
                true
            } else if info.starlark_semantics.contains("enable_bzlmod=false") {
                info!("found enable_bzlmod=false in starlark-semantics");
                false
            } else {
                bzlmod_enabled_by_default
            }
        };

        info!("bzlmod_enabled = {}", bzlmod_enabled);

        // Load builtin rules from `bazel info build-language`.
        debug!("fetching builtin rules via `bazel info build-language`");
        let rules = match load_bazel_build_language(&*bazel_client) {
            Ok(builtins) => {
                debug!("successfully fetched builtin rules");
                builtins
            }
            Err(err) => {
                error!("failed to run `bazel info build-language`: {}", err);
                has_bazel_init_err = true;
                Default::default()
            }
        };

        // Query for all targets in the current workspace, to use for label completion.
        let targets = if config.args.enable_label_completions {
            debug!("querying for all targets in the current workspace");
            match bazel_client.query_all_workspace_targets() {
                Ok(targets) => {
                    debug!("successfully queried for all targets");
                    targets
                }
                Err(err) => {
                    error!("failed to query all workspace targets: {}", err);
                    has_bazel_init_err = true;
                    Default::default()
                }
            }
        } else {
            Default::default()
        };

        let path_interner = Arc::new(PathInterner::default());
        let loader = DefaultFileLoader::new(
            bazel_client.clone(),
            path_interner.clone(),
            info.workspace.clone(),
            info.workspace_name,
            external_output_base.clone(),
            task_pool_sender.clone(),
            bzlmod_enabled,
        );
        let mut analysis = Analysis::new(
            Arc::new(loader),
            InferenceOptions {
                infer_ctx_attributes: config.args.infer_ctx_attributes,
                use_code_flow_analysis: config.args.use_code_flow_analysis,
                report_unused_definitions: true,
            },
        );

        analysis.set_all_workspace_targets(targets);
        analysis.set_builtin_defs(builtins, rules);

        // Check for a prelude file. We skip verifying that `//tools/build_tools` is actually a package (i.e.
        // that it actually contains a `BUILD.bazel`) file for simplicity.
        if let Ok((prelude, contents)) = load_bazel_prelude(&info.workspace) {
            info!("found prelude file at {:?}", prelude);
            let file_id = path_interner.intern_path(prelude);
            let mut change = Change::default();
            change.create_file(
                file_id,
                Dialect::Bazel,
                Some(FileInfo::Bazel {
                    api_context: APIContext::Bzl,
                    is_external: false,
                }),
                contents,
            );
            analysis.apply_change(change);
            analysis.set_bazel_prelude_file(file_id);
        }

        let server = Server {
            config: Arc::new(config),
            connection,
            req_queue: Default::default(),
            task_pool_handle,
            document_manager: Arc::new(RwLock::new(DocumentManager::new(
                path_interner,
                info.workspace,
            ))),
            diagnostics_manager: Default::default(),
            analysis,
            analysis_debouncer: AnalysisDebouncer::new(DEBOUNCE_INTERVAL, task_pool_sender),
            analysis_requested_for_files: None,
            bazel_client,
            pending_repos: Default::default(),
            pending_files: Default::default(),
            force_analysis_for_files: Default::default(),
            fetched_repos: Default::default(),
            is_fetching_repos: false,
            is_refreshing_all_workspace_targets: false,
            bzlmod_enabled,
        };

        if has_bazel_init_err {
            server.send_error_message(BAZEL_INIT_ERR_MESSAGE);
        }

        Ok(server)
    }

    pub(crate) fn snapshot(&self) -> ServerSnapshot {
        ServerSnapshot {
            config: self.config.clone(),
            analysis_snapshot: self.analysis.snapshot(),
            document_manager: Arc::clone(&self.document_manager),
        }
    }

    pub(crate) fn process_changes(&mut self) -> (Vec<FileId>, bool) {
        let mut change = Change::default();
        let mut document_manager = self.document_manager.write();
        let (has_opened_or_closed_documents, changes) = document_manager.take_changes();
        let changed_file_ids = changes.iter().map(|(file_id, _)| *file_id).collect();

        if changes.is_empty() && self.force_analysis_for_files.is_empty() {
            return (changed_file_ids, has_opened_or_closed_documents);
        }

        let mut prelude_file = None;

        for (file_id, change_kind) in changes {
            let document = match document_manager.get(file_id) {
                Some(document) => document,
                None => continue,
            };
            match change_kind {
                DocumentChangeKind::Create => {
                    if matches!(
                        document.info,
                        Some(FileInfo::Bazel {
                            api_context: APIContext::Prelude,
                            ..
                        })
                    ) {
                        prelude_file = Some(file_id)
                    }

                    change.create_file(
                        file_id,
                        document.dialect,
                        document.info.clone(),
                        document.contents.clone(),
                    );
                }
                DocumentChangeKind::Update => {
                    change.update_file(file_id, document.contents.clone());
                }
            }
        }

        drop(document_manager);

        // Apply the change to our analyzer. This will cancel any affected active Salsa operations.
        self.analysis.apply_change(change);
        if let Some(prelude_file) = prelude_file {
            self.analysis.set_bazel_prelude_file(prelude_file);
        }

        (changed_file_ids, true)
    }

    pub(crate) fn send_request<R: lsp_types::request::Request>(&mut self, params: R::Params) {
        let req = self
            .req_queue
            .outgoing
            .register(R::METHOD.to_string(), params, ());
        self.send(req.into());
    }

    pub(crate) fn complete_request(&mut self, resp: lsp_server::Response) {
        self.req_queue.outgoing.complete(resp.id);
    }

    pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
        &self,
        params: N::Params,
    ) {
        let not = lsp_server::Notification::new(N::METHOD.to_string(), params);
        self.send(not.into());
    }

    pub(crate) fn send(&self, message: lsp_server::Message) {
        self.connection.sender.send(message).unwrap();
    }

    pub(crate) fn send_error_message(&self, message: &str) {
        self.send_notification::<lsp_types::notification::ShowMessage>(
            lsp_types::ShowMessageParams {
                message: message.to_string(),
                typ: lsp_types::MessageType::ERROR,
            },
        )
    }

    pub(crate) fn fetch_bazel_external_repos(&mut self) {
        let repos = mem::take(&mut self.pending_repos);
        let files = mem::take(&mut self.pending_files);
        let bazel_client = self.bazel_client.clone();
        let bzlmod_enabled = self.bzlmod_enabled;

        self.is_fetching_repos = true;
        self.fetched_repos.extend(repos.clone());
        self.task_pool_handle.spawn_with_sender(move |sender| {
            sender
                .send(Task::FetchExternalRepos(FetchExternalReposProgress::Begin(
                    repos.clone(),
                )))
                .unwrap();

            let mut failed_repos = vec![];

            for repo in &repos {
                debug!("fetching external repository \"@@{}\"", repo);
                if let Err(err) = if bzlmod_enabled {
                    bazel_client.fetch_repo(repo)
                } else {
                    bazel_client.null_query_external_repo_targets(repo)
                } {
                    failed_repos.push(repo.clone());
                    error!(
                        "failed to fetch external repository \"@@{}\": {}",
                        repo, err
                    );
                }
            }

            sender
                .send(Task::FetchExternalRepos(FetchExternalReposProgress::End(
                    files,
                    failed_repos,
                )))
                .unwrap();
        });
    }

    pub(crate) fn refresh_all_workspace_targets(&mut self) {
        if self.is_refreshing_all_workspace_targets || !self.config.args.enable_label_completions {
            return;
        }

        let bazel_client = self.bazel_client.clone();

        self.is_refreshing_all_workspace_targets = true;
        self.task_pool_handle.spawn_with_sender(move |sender| {
            sender
                .send(Task::RefreshAllWorkspaceTargets(
                    RefreshAllWorkspaceTargetsProgress::Begin,
                ))
                .unwrap();

            let targets = match bazel_client.query_all_workspace_targets() {
                Ok(targets) => Some(targets),
                Err(err) => {
                    error!("failed to query all workspace targets: {}", err);
                    None
                }
            };

            sender
                .send(Task::RefreshAllWorkspaceTargets(
                    RefreshAllWorkspaceTargetsProgress::End(targets),
                ))
                .unwrap();
        });
    }
}

impl panic::RefUnwindSafe for ServerSnapshot {}

pub(crate) fn load_bazel_builtins() -> anyhow::Result<Builtins> {
    let data = include_bytes!("builtin/builtin.pb");
    let builtins = decode_builtins(&data[..])?;
    Ok(builtins)
}

pub(crate) fn load_bazel_build_language(client: &dyn BazelClient) -> anyhow::Result<Builtins> {
    let build_language_output = client.build_language()?;
    decode_rules(&build_language_output)
}

fn load_bazel_prelude(workspace: impl AsRef<Path>) -> anyhow::Result<(PathBuf, String)> {
    let prelude = workspace.as_ref().join("tools/build_rules/prelude_bazel");
    let contents = fs::read_to_string(&prelude)?;
    Ok((prelude, contents))
}
