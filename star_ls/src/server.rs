use lsp_server::{Connection, ReqQueue};

use crate::document::DocumentManager;

pub(crate) struct Server {
    pub(crate) connection: Connection,
    pub(crate) req_queue: ReqQueue<(), ()>,
    pub(crate) document_manager: DocumentManager,
}

impl Server {
    pub(crate) fn new(connection: Connection) -> anyhow::Result<Self> {
        Ok(Server {
            connection,
            req_queue: Default::default(),
            document_manager: Default::default(),
        })
    }
}
