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

use crate::bazel::BazelContext;
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

        // Check if the user specified a path to the Bazel executable.
        let bazel_path = config
            .args
            .bazel_path
            .clone()
            .unwrap_or("bazel".to_string());

        debug!(
            "fetching Bazel configuration using Bazel executable at {:?}",
            bazel_path
        );

        // Determine Bazel configuration.
        let mut has_bazel_init_err = false;
        let bazel_client = Arc::new(BazelCLI::new(&bazel_path));
        let bazel_cx = match BazelContext::new(&*bazel_client) {
            Ok(cx) => cx,
            Err(err) => {
                has_bazel_init_err = true;
                error!("failed to initialize Bazel context: {}", err);
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

        // Load extensions from JSON files
        let extensions = if !config.args.extension_files.is_empty() {
            info!("Loading extensions...");

            match starpls_common::load_extensions(&config.args.extension_files) {
                Ok(extensions) => {
                    info!("✓ Loaded extension(s) successfully");
                    Arc::new(extensions)
                }
                Err(e) => {
                    error!("Failed to load extensions: {}", e);
                    return Err(e);
                }
            }
        } else {
            Arc::new(starpls_common::Extensions::new())
        };

        let path_interner = Arc::new(PathInterner::default());
        let loader = DefaultFileLoader::new(
            bazel_client.clone(),
            path_interner.clone(),
            bazel_cx.info.workspace.clone(),
            bazel_cx.info.workspace_name,
            bazel_cx.info.output_base.join("external"),
            task_pool_sender.clone(),
            bazel_cx.bzlmod_enabled,
        )
        .with_extensions(extensions.clone());
        let mut analysis = Analysis::new(
            Arc::new(loader),
            InferenceOptions {
                infer_ctx_attributes: config.args.inference_options.infer_ctx_attributes,
                use_code_flow_analysis: config.args.inference_options.use_code_flow_analysis,
                ..Default::default()
            },
        );

        analysis.set_all_workspace_targets(targets);

        // Load base builtins and inject extension global symbols
        let mut builtins = load_bazel_builtins();
        inject_extension_globals(&mut builtins, &extensions);
        process_extension_types_and_globals(&mut builtins, &extensions);
        analysis.set_builtin_defs(builtins, bazel_cx.rules);

        // Check for a prelude file. We skip verifying that `//tools/build_tools` is actually a package (i.e.
        // that it actually contains a `BUILD.bazel`) file for simplicity.
        if let Ok((prelude, contents)) = load_bazel_prelude(&bazel_cx.info.workspace) {
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

        let analysis_debounce_interval = config.args.analysis_debounce_interval;
        let server = Server {
            config: Arc::new(config),
            connection,
            req_queue: Default::default(),
            task_pool_handle,
            document_manager: Arc::new(RwLock::new(DocumentManager::new(
                path_interner,
                bazel_cx.info.workspace,
            ))),
            diagnostics_manager: Default::default(),
            analysis,
            analysis_debouncer: AnalysisDebouncer::new(
                Duration::from_millis(analysis_debounce_interval),
                task_pool_sender,
            ),
            analysis_requested_for_files: None,
            bazel_client,
            pending_repos: Default::default(),
            pending_files: Default::default(),
            force_analysis_for_files: Default::default(),
            fetched_repos: Default::default(),
            is_fetching_repos: false,
            is_refreshing_all_workspace_targets: false,
            bzlmod_enabled: bazel_cx.bzlmod_enabled,
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

pub(crate) fn load_bazel_builtins() -> Builtins {
    let data = include_bytes!("builtin/builtin.pb");

    // We want to crash if the bundled protobuf file is ever invalid.
    decode_builtins(&data[..]).expect("bug: invalid builtin.pb")
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

/// Inject global symbols from extensions into the builtin definitions.
/// This allows extension-defined symbols to be available without explicit load() statements.
fn inject_extension_globals(builtins: &mut Builtins, extensions: &starpls_common::Extensions) {
    use starpls_bazel::builtin::Callable;
    use starpls_bazel::builtin::Param;
    use starpls_bazel::builtin::Value;

    for symbol in extensions.global_symbols() {
        let value = if let Some(callable) = &symbol.callable {
            // This is a function
            let builtin_params = callable
                .params
                .iter()
                .map(|p| Param {
                    name: p.name.clone(),
                    r#type: p.r#type.clone(),
                    doc: p.doc.clone(),
                    default_value: p.default_value.clone(),
                    is_mandatory: p.is_mandatory,
                    is_star_arg: p.is_star_arg,
                    is_star_star_arg: p.is_star_star_arg,
                })
                .collect();

            Value {
                name: symbol.name.clone(),
                r#type: symbol.r#type.clone(),
                doc: symbol.doc.clone(),
                callable: Some(Callable {
                    param: builtin_params,
                    return_type: callable.return_type.clone(),
                }),
                ..Default::default()
            }
        } else {
            // This is a variable
            Value {
                name: symbol.name.clone(),
                r#type: symbol.r#type.clone(),
                doc: symbol.doc.clone(),
                callable: None,
                ..Default::default()
            }
        };

        builtins.global.push(value);
    }
}


/// Process an extension symbol that defines a type, converting it to proper type system objects.
/// Returns (type_name, protobuf_type_definition) for registration in the builtin system.
fn process_extension_symbol_as_type(
    symbol: &starpls_common::Symbol,
) -> Option<(String, starpls_bazel::builtin::Type)> {
    use starpls_bazel::builtin::{Type, Value, Callable, Param};

    if !symbol.is_type_definition() {
        return None;
    }

    // Convert properties to protobuf Value objects
    let mut fields = Vec::new();

    for (prop_name, prop_symbol) in &symbol.properties {
        let value = if let Some(callable) = &prop_symbol.callable {
            // It's a method
            let builtin_params = callable
                .params
                .iter()
                .map(|p| Param {
                    name: p.name.clone(),
                    r#type: p.r#type.clone(),
                    doc: p.doc.clone(),
                    default_value: p.default_value.clone(),
                    is_mandatory: p.is_mandatory,
                    is_star_arg: p.is_star_arg,
                    is_star_star_arg: p.is_star_star_arg,
                })
                .collect();

            Value {
                name: prop_name.clone(),
                r#type: prop_symbol.r#type.clone(),
                doc: prop_symbol.doc.clone(),
                callable: Some(Callable {
                    param: builtin_params,
                    return_type: callable.return_type.clone(),
                }),
                ..Default::default()
            }
        } else {
            // It's a field
            Value {
                name: prop_name.clone(),
                r#type: prop_symbol.r#type.clone(),
                doc: prop_symbol.doc.clone(),
                callable: None,
                ..Default::default()
            }
        };

        fields.push(value);
    }

    // Create the Type definition
    let type_def = Type {
        name: symbol.name.clone(),
        field: fields,
        doc: symbol.doc.clone(),
    };

    Some((symbol.name.clone(), type_def))
}

/// Process extension symbols that should be registered as both types and globals (self-referential pattern).
///
/// This function implements the critical self-referential pattern that makes extension objects
/// work like Bazel built-ins such as `apple_common`, `native`, etc.
///
/// ## The Self-Referential Pattern Explained
///
/// Objects like `apple_common` work in Bazel because they use a dual registration:
///
/// 1. **Type Registration**: Creates a `BuiltinType` with all the object's fields and methods
/// 2. **Global Registration**: Creates a global variable with the same name pointing to that type
///
/// This allows the type system to understand both:
/// - That `apple_common` exists as a global symbol
/// - What fields/methods are available on `apple_common` (like `.apple_toolchain`, `.platform`)
///
/// ## Why This Matters for Extensions
///
/// Without this pattern, extensions create `struct()` calls in virtual files, which results in:
/// - Generic `TyKind::Struct` types that the type system can't analyze
/// - No field information preserved (everything becomes "Unknown")
/// - Broken LSP features (no completion, no hover docs, no type checking)
///
/// With this pattern, extensions create proper `BuiltinType` instances, enabling:
/// - Full type information and documentation
/// - Working completion for object members
/// - Proper hover information showing parameter types and docs
/// - Type checking that validates method calls and parameters
///
/// ## Implementation Details
///
/// For each extension symbol with `as_type: true`:
/// 1. Convert the symbol's properties to a protobuf `Type` definition
/// 2. Register it in `builtins.type` (makes it a known type)
/// 3. Register a global `Value` with `name: "symbol"` and `type: "symbol"` (self-referential)
/// 4. The type system now knows both the global and its structure
fn process_extension_types_and_globals(
    builtins: &mut Builtins,
    extensions: &starpls_common::Extensions,
) {
    use starpls_bazel::builtin::Value;

    // First, collect all type definitions from modules
    let mut type_definitions = std::collections::HashMap::new();

    for (_, symbol) in extensions.all_module_symbols() {
        if let Some((type_name, type_def)) = process_extension_symbol_as_type(symbol) {
            type_definitions.insert(type_name, type_def);
        }
    }

    // Process global symbols that should be registered as types
    for symbol in extensions.global_symbols() {
        if symbol.should_register_as_type() {
            // Add type definition if we have it
            if let Some((_, type_def)) = process_extension_symbol_as_type(symbol) {
                builtins.r#type.push(type_def);
            }

            // Add self-referential global
            let global_value = Value {
                name: symbol.name.clone(),
                r#type: symbol.name.clone(), // Self-referential!
                doc: symbol.doc.clone(),
                callable: None,
                ..Default::default()
            };
            builtins.global.push(global_value);
        }
    }

    // Add any remaining type definitions that aren't self-referential globals
    for (_, type_def) in type_definitions {
        builtins.r#type.push(type_def);
    }
}
