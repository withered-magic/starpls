use crate::{
    event_loop::Task,
    server::{Server, ServerSnapshot},
};
use starpls_ide::Cancelled;
use std::panic;

pub(crate) struct RequestDispatcher<'a> {
    req: Option<lsp_server::Request>,
    server: &'a Server,
}

impl<'a> RequestDispatcher<'a> {
    pub(crate) fn new(req: lsp_server::Request, server: &'a Server) -> Self {
        Self {
            req: Some(req),
            server,
        }
    }

    pub(crate) fn on<R>(
        &mut self,
        f: fn(&ServerSnapshot, R::Params) -> anyhow::Result<R::Result>,
    ) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: serde::de::DeserializeOwned + Send + panic::UnwindSafe,
    {
        let (req, params) = match self.parse::<R>() {
            Some(res) => res,
            None => return self,
        };

        let snapshot = self.server.snapshot();
        self.server.task_pool_handle.spawn(move || {
            let res = panic::catch_unwind(|| f(&snapshot, params));
            let response = match res {
                Ok(res) => match res {
                    Ok(res) => lsp_server::Response::new_ok(req.id, res),
                    Err(err) => match err.downcast::<Cancelled>() {
                        Ok(_) => return Task::Retry(req),
                        Err(err) => lsp_server::Response::new_err(
                            req.id,
                            lsp_server::ErrorCode::RequestFailed as i32,
                            err.to_string(),
                        ),
                    },
                },
                Err(err) => {
                    let panic_message = err
                        .downcast_ref::<String>()
                        .map(String::as_str)
                        .or_else(|| err.downcast_ref::<&str>().copied());
                    lsp_server::Response::new_err(
                        req.id,
                        lsp_server::ErrorCode::RequestFailed as i32,
                        format!(
                            "request handler panicked: {}",
                            panic_message.unwrap_or("unknown reason")
                        ),
                    )
                }
            };

            Task::ResponseReady(response)
        });

        self
    }

    pub(crate) fn finish(&mut self) {
        let req = match self.req.take() {
            Some(req) => req,
            None => return,
        };

        self.server.task_pool_handle.spawn(move || {
            Task::ResponseReady(lsp_server::Response::new_err(
                req.id,
                lsp_server::ErrorCode::MethodNotFound as i32,
                format!("method not found: {}", req.method),
            ))
        });
    }

    pub(crate) fn parse<R>(&mut self) -> Option<(lsp_server::Request, R::Params)>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
    {
        self.req.take().and_then(|req| {
            if req.method == R::METHOD {
                // Unwrapping here is fine, since if we see invalid JSON, we can't really recover parsing afterwards.
                let params = serde_json::from_value(req.params.clone()).expect("invalid JSON");
                Some((req, params))
            } else {
                self.req = Some(req);
                None
            }
        })
    }
}
