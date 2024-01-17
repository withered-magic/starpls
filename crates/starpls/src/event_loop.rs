use crate::{
    convert,
    document::DocumentSource,
    extensions,
    handlers::{notifications, requests},
    server::{Server, ServerSnapshot},
};
use crossbeam_channel::select;
use lsp_server::Connection;
use lsp_types::InitializeParams;
use starpls_common::FileId;

#[macro_export]
macro_rules! match_notification {
    (match $node:ident { $($tt:tt)* }) => { $crate::match_notification!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( if $path:path as $it:pat => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = cast_notification::<$path>(&$node) { $res } else )*
        { $catch_all }
    }};
}

#[derive(Debug)]
pub(crate) enum Task {
    AnalysisRequested,
    /// A new set of diagnostics has been processed and is ready for forwarding.
    DiagnosticsReady(Vec<(FileId, Vec<lsp_types::Diagnostic>)>),
    /// A request has been evaluated and its response is ready.
    ResponseReady(lsp_server::Response),
}

#[derive(Debug)]
pub(crate) enum Event {
    Message(lsp_server::Message),
    Task(Task),
}

pub fn process_connection(
    connection: Connection,
    _initialize_params: InitializeParams,
) -> anyhow::Result<()> {
    eprintln!("server: initializing state and starting event loop");
    let server = Server::new(connection)?;
    server.run()
}

impl Server {
    fn run(mut self) -> anyhow::Result<()> {
        while let Some(event) = self.next_event() {
            self.handle_event(event)?;
        }
        Ok(())
    }

    fn next_event(&self) -> Option<Event> {
        let event = select! {
            recv(self.connection.receiver) -> req => req.ok().map(Event::Message),
            recv(self.task_pool_handle.receiver) -> task => Some(Event::Task(task.unwrap())),
        };
        event
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Message(lsp_server::Message::Request(req)) => {
                self.register_and_handle_request(req);
            }
            Event::Message(lsp_server::Message::Notification(not)) => {
                self.handle_notification(not)?;
            }
            Event::Task(task) => self.handle_task(task),
            _ => (),
        };

        // Update our diagnostics if a triggering event (e.g. document open/close/change) occured.
        // This is done asynchronously, so any new diagnostics resulting from this won't be seen until the next turn
        // of the event loop.
        if self.process_changes() {
            self.analysis_requested = false;
            self.analysis_debouncer.sender.send(()).unwrap();
        } else if self.analysis_requested {
            self.update_diagnostics();
        }

        let changed_file_ids = self.diagnostics_manager.take_changes();

        for file_id in changed_file_ids {
            let document_manager = self.document_manager.read();
            // Only send diagnostics for currently open editors.
            let version = match document_manager
                .get(file_id)
                .map(|document| document.source)
            {
                Some(DocumentSource::Editor(version)) => version,
                _ => continue,
            };
            let diagnostics = self
                .diagnostics_manager
                .get_diagnostics(file_id)
                .cloned()
                .collect::<Vec<_>>();
            let path = document_manager.lookup_by_file_id(file_id);
            let uri = lsp_types::Url::from_file_path(path).unwrap();

            drop(document_manager);

            self.send_notification::<lsp_types::notification::PublishDiagnostics>(
                lsp_types::PublishDiagnosticsParams {
                    uri,
                    diagnostics,
                    version: Some(version),
                },
            );
        }

        Ok(())
    }

    fn update_diagnostics(&mut self) {
        let file_ids = self
            .document_manager
            .read()
            .iter()
            .map(|(path, _)| path.clone())
            .collect::<Vec<_>>();
        let snapshot = self.snapshot();
        self.task_pool_handle.spawn(move || {
            let mut res = Vec::new();

            // Query the database for diagnostics for each file and convert them to an LSP-compatible format.
            for file_id in file_ids {
                let diagnostics = match collect_diagnostics(&snapshot, file_id) {
                    Some(diagnositcs) => diagnositcs,
                    None => continue,
                };
                res.push((file_id, diagnostics));
            }

            Task::DiagnosticsReady(res)
        });
        self.analysis_requested = false;
    }

    fn register_and_handle_request(&mut self, req: lsp_server::Request) {
        self.req_queue.incoming.register(req.id.clone(), ());
        self.handle_request(req);
    }

    fn handle_request(&mut self, req: lsp_server::Request) {
        // TODO: Refactor this into a `RequestDispatcher`.
        if let Some(params) = cast_request::<extensions::ShowSyntaxTree>(&req) {
            let snapshot = self.snapshot();
            self.task_pool_handle.spawn(move || {
                let id = req.id.clone();
                Task::ResponseReady(match requests::show_syntax_tree(&snapshot, params) {
                    Ok(value) => lsp_server::Response::new_ok(id, value),
                    Err(err) => lsp_server::Response::new_err(
                        id,
                        lsp_server::ErrorCode::RequestFailed as i32,
                        err.to_string(),
                    ),
                })
            });
        } else if let Some(params) = cast_request::<extensions::ShowHir>(&req) {
            let snapshot = self.snapshot();
            self.task_pool_handle.spawn(move || {
                let id = req.id.clone();
                Task::ResponseReady(match requests::show_hir(&snapshot, params) {
                    Ok(value) => lsp_server::Response::new_ok(id, value),
                    Err(err) => lsp_server::Response::new_err(
                        id,
                        lsp_server::ErrorCode::RequestFailed as i32,
                        err.to_string(),
                    ),
                })
            });
        } else if let Some(params) = cast_request::<lsp_types::request::GotoDefinition>(&req) {
            let snapshot = self.snapshot();
            self.task_pool_handle.spawn(move || {
                let id = req.id.clone();
                Task::ResponseReady(match requests::goto_definition(&snapshot, params) {
                    Ok(value) => lsp_server::Response::new_ok(id, value),
                    Err(err) => lsp_server::Response::new_err(
                        id,
                        lsp_server::ErrorCode::RequestFailed as i32,
                        err.to_string(),
                    ),
                })
            });
        } else if let Some(params) = cast_request::<lsp_types::request::Completion>(&req) {
            let snapshot = self.snapshot();
            self.task_pool_handle.spawn(move || {
                let id = req.id.clone();
                Task::ResponseReady(match requests::completion(&snapshot, params) {
                    Ok(value) => lsp_server::Response::new_ok(id, value),
                    Err(err) => lsp_server::Response::new_err(
                        id,
                        lsp_server::ErrorCode::RequestFailed as i32,
                        err.to_string(),
                    ),
                })
            });
        } else if let Some(params) = cast_request::<lsp_types::request::HoverRequest>(&req) {
            let snapshot = self.snapshot();
            self.task_pool_handle.spawn(move || {
                let id = req.id.clone();
                Task::ResponseReady(match requests::hover(&snapshot, params) {
                    Ok(value) => lsp_server::Response::new_ok(id, value),
                    Err(err) => lsp_server::Response::new_err(
                        id,
                        lsp_server::ErrorCode::RequestFailed as i32,
                        err.to_string(),
                    ),
                })
            });
        }
    }

    fn handle_notification(&mut self, not: lsp_server::Notification) -> anyhow::Result<()> {
        match_notification! {
            match not {
                if lsp_types::notification::DidOpenTextDocument as params => notifications::did_open_text_document(self, params),
                if lsp_types::notification::DidCloseTextDocument as params => notifications::did_close_text_document(self, params),
                if lsp_types::notification::DidChangeTextDocument as params => notifications::did_change_text_document(self, params),
                _ => Ok(())
            }
        }
    }

    fn handle_task(&mut self, task: Task) {
        match task {
            Task::AnalysisRequested => self.analysis_requested = true,
            Task::DiagnosticsReady(diagnostics) => {
                for (file_id, diagnostics) in diagnostics {
                    self.diagnostics_manager
                        .set_diagnostics(file_id, diagnostics);
                }
            }
            Task::ResponseReady(resp) => {
                self.respond(resp);
            }
        }
    }

    fn respond(&mut self, resp: lsp_server::Response) {
        if self.req_queue.incoming.complete(resp.id.clone()).is_some() {
            self.connection.sender.send(resp.into()).unwrap();
        }
    }
}

fn cast_request<R>(req: &lsp_server::Request) -> Option<R::Params>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    if req.method == R::METHOD {
        // Unwrapping here is fine, since if we see invalid JSON, we can't really recover parsing afterwards.
        let params = serde_json::from_value(req.params.clone()).expect("invalid JSON");
        Some(params)
    } else {
        None
    }
}

fn cast_notification<R>(not: &lsp_server::Notification) -> Option<R::Params>
where
    R: lsp_types::notification::Notification,
    R::Params: serde::de::DeserializeOwned,
{
    if not.method == R::METHOD {
        let params = serde_json::from_value(not.params.clone()).expect("invalid JSON");
        Some(params)
    } else {
        None
    }
}

fn collect_diagnostics(
    snapshot: &ServerSnapshot,
    file_id: FileId,
) -> Option<Vec<lsp_types::Diagnostic>> {
    let line_index = snapshot.analysis_snapshot.line_index(file_id).ok()??;

    // Get the diagnostics for the current path. If the operation was cancelled, simply continue to the next file.
    let diagnostics = snapshot.analysis_snapshot.diagnostics(file_id).ok()?;

    // Convert the diagnostics. This includes translating text offsets into `(line, column)` format.
    Some(
        diagnostics
            .into_iter()
            .map(|diagnostic| convert::lsp_diagnostic_from_native(diagnostic, &line_index))
            .collect::<Vec<_>>(),
    )
}
