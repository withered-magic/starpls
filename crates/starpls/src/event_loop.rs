use crate::{handlers::notifications, server::Server};
use crossbeam_channel::select;
use lsp_server::Connection;
use lsp_types::InitializeParams;

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

#[macro_export]
macro_rules! match_request {
    (match $node:ident { $($tt:tt)* }) => { $crate::match_request!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( if $path:path as $it:pat => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = cast_request::<$path>(&$node) { $res } else )*
        { $catch_all }
    }};
}

#[derive(Debug)]
pub(crate) enum Task {
    /// A new set of diagnostics has been processed and is ready for forwarding.
    DiagnosticsReady(Vec<(u32, Vec<lsp_types::Diagnostic>)>),
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
        };
        event
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Message(lsp_server::Message::Notification(not)) => {
                self.handle_notification(not)?;
            }
            _ => (),
        };

        // Update our diagnostics if a triggering event (e.g. document open/close/change) occured.
        // This is done asynchronously, so any new diagnostics resulting from this won't be seen until the next turn
        // of the event loop.
        if self.process_changes() {
            self.update_diagnostics();
        }

        Ok(())
    }

    fn register_and_handle_request(&mut self, req: lsp_server::Request) {
        self.req_queue.incoming.register(req.id.clone(), ());
        self.handle_request(req);
    }

    fn handle_request(&mut self, req: lsp_server::Request) {
        let _snapshot = self.snapshot();
        self.task_pool_handle.spawn(move || {
            let id = req.id.clone();
            let _res: anyhow::Result<()> = match_request! {
                match req {
                    _ => Ok(())
                }
            };
            Task::ResponseReady(lsp_server::Response::new_err(
                id,
                lsp_server::ErrorCode::InternalError as i32,
                "unimplemented".to_string(),
            ))
        });
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
