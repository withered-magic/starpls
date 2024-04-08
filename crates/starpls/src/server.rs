use crate::{
    debouncer::AnalysisDebouncer,
    diagnostics::DiagnosticsManager,
    document::{DefaultFileLoader, DocumentChangeKind, DocumentManager, PathInterner},
    event_loop::{FetchBazelExternalReposProgress, Task},
    task_pool::{TaskPool, TaskPoolHandle},
};
use lsp_server::{Connection, ReqQueue};
use parking_lot::RwLock;
use starpls_bazel::{
    build_language::decode_rules,
    client::{BazelCLI, BazelClient},
    decode_builtins, Builtins,
};
use starpls_common::FileId;
use starpls_ide::{Analysis, AnalysisSnapshot, Change};
use std::{panic, sync::Arc, time::Duration};

const DEBOUNCE_INTERVAL: Duration = Duration::from_millis(250);

pub(crate) struct Server {
    pub(crate) connection: Connection,
    pub(crate) req_queue: ReqQueue<(), ()>,
    pub(crate) task_pool_handle: TaskPoolHandle<Task>,
    pub(crate) document_manager: Arc<RwLock<DocumentManager>>,
    pub(crate) diagnostics_manager: DiagnosticsManager,
    pub(crate) analysis: Analysis,
    pub(crate) analysis_debouncer: AnalysisDebouncer,
    pub(crate) analysis_requested_for_files: Option<Vec<FileId>>,
    pub(crate) fetch_bazel_external_repos_requested: bool,
    pub(crate) bazel_client: Arc<dyn BazelClient>,
}

pub(crate) struct ServerSnapshot {
    pub(crate) analysis_snapshot: AnalysisSnapshot,
    pub(crate) document_manager: Arc<RwLock<DocumentManager>>,
}

impl Server {
    pub(crate) fn new(connection: Connection) -> anyhow::Result<Self> {
        // Create the task pool for processin incoming requests.
        let (sender, receiver) = crossbeam_channel::unbounded();
        let task_pool = TaskPool::with_num_threads(sender.clone(), 4)?;
        let task_pool_handle = TaskPoolHandle::new(receiver, task_pool);

        // Load Bazel builtins from the specified file.
        let builtins = match load_bazel_builtins() {
            Ok(builtins) => builtins,
            Err(err) => {
                eprintln!("server: failed to load builtins, {}", err);
                Default::default()
            }
        };

        let bazel_client = Arc::new(BazelCLI::default());

        // Determine the workspace root.
        let workspace = bazel_client.workspace().unwrap_or_else(|err| {
            eprintln!("server: failed to run `bazel info workspace`: {}", err);
            Default::default()
        });

        eprintln!("server: workspace root: {:?}", workspace);

        // Determine the output base for the purpose of resolving external repositories.
        let external_output_base = bazel_client
            .output_base()
            .map(|output_base| output_base.join("external"))
            .unwrap_or_else(|err| {
                eprintln!("server: failed to run `bazel info output_base`: {}", err);
                Default::default()
            });

        eprintln!("server: external output base: {:?}", external_output_base);

        let bzlmod_enabled = workspace.join("MODULE.bazel").try_exists().unwrap_or(false)
            && {
                eprintln!("server: checking for `bazel mod dump_repo_mapping` capability");
                match bazel_client.dump_repo_mapping("") {
                    Ok(_) => true,
                    Err(_) => {
                        eprintln!("server: installed Bazel version doesn't support `bazel mod dump_repo_mapping`, disabling bzlmod support");
                        false
                    }
                }
            };

        eprintln!("server: bzlmod_enabled = {}", bzlmod_enabled);

        // Additionally, load builtin rules.
        eprintln!("server: fetching builtin rules via `bazel info build-language`");
        let rules = match load_bazel_build_language(&*bazel_client) {
            Ok(builtins) => builtins,
            Err(err) => {
                eprintln!("server: failed to run `bazel info build-language`: {}", err);
                Default::default()
            }
        };

        let path_interner = Arc::new(PathInterner::default());
        let loader = DefaultFileLoader::new(
            bazel_client.clone(),
            path_interner.clone(),
            workspace,
            external_output_base,
            bzlmod_enabled,
        );

        let mut analysis = Analysis::new(Arc::new(loader));
        analysis.set_builtin_defs(builtins, rules);

        Ok(Server {
            connection,
            req_queue: Default::default(),
            task_pool_handle,
            document_manager: Arc::new(RwLock::new(DocumentManager::new(path_interner))),
            diagnostics_manager: Default::default(),
            analysis,
            analysis_debouncer: AnalysisDebouncer::new(DEBOUNCE_INTERVAL, sender),
            analysis_requested_for_files: None,
            fetch_bazel_external_repos_requested: false,
            bazel_client,
        })
    }

    pub(crate) fn snapshot(&self) -> ServerSnapshot {
        ServerSnapshot {
            analysis_snapshot: self.analysis.snapshot(),
            document_manager: Arc::clone(&self.document_manager),
        }
    }

    pub(crate) fn process_changes(&mut self) -> (Vec<FileId>, bool) {
        let mut change = Change::default();
        let mut document_manager = self.document_manager.write();
        let (has_opened_or_closed_documents, changes) = document_manager.take_changes();
        let changed_file_ids = changes.iter().map(|(file_id, _)| *file_id).collect();

        if changes.is_empty() {
            return (changed_file_ids, has_opened_or_closed_documents);
        }

        for (file_id, change_kind) in changes {
            let document = match document_manager.get(file_id) {
                Some(document) => document,
                None => continue,
            };
            match change_kind {
                DocumentChangeKind::Create => {
                    change.create_file(file_id, document.dialect, document.contents.clone());
                }
                DocumentChangeKind::Update => {
                    change.update_file(file_id, document.contents.clone());
                }
            }
        }

        drop(document_manager);

        // Apply the change to our analyzer. This will cancel any affected active Salsa operations.
        self.analysis.apply_change(change);
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

    #[allow(unused)]
    pub(crate) fn fetch_bazel_external_repos(&mut self) {
        let bazel_client = self.bazel_client.clone();
        self.fetch_bazel_external_repos_requested = true;
        self.task_pool_handle.spawn_with_sender(move |sender| {
            sender
                .send(Task::FetchBazelExternalRepos(
                    FetchBazelExternalReposProgress::Begin,
                ))
                .unwrap();

            let res = load_bazel_build_language(&*bazel_client);
            sender
                .send(Task::FetchBazelExternalRepos(
                    FetchBazelExternalReposProgress::End(res),
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
