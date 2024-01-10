use crate::{
    diagnostics::DiagnosticsManager,
    document::DocumentManager,
    event_loop::Task,
    task_pool::{TaskPool, TaskPoolHandle},
};
use lsp_server::{Connection, ReqQueue};
use parking_lot::RwLock;
use starpls_ide::{Analysis, AnalysisSnapshot, Change};
use std::sync::Arc;

pub(crate) struct Server {
    pub(crate) connection: Connection,
    pub(crate) req_queue: ReqQueue<(), ()>,
    pub(crate) task_pool_handle: TaskPoolHandle<Task>,
    pub(crate) document_manager: Arc<RwLock<DocumentManager>>,
    pub(crate) diagnostics_manager: DiagnosticsManager,
    pub(crate) analysis: Analysis,
}

pub(crate) struct ServerSnapshot {
    pub(crate) analysis_snapshot: AnalysisSnapshot,
    pub(crate) document_manager: Arc<RwLock<DocumentManager>>,
}

impl Server {
    pub(crate) fn new(connection: Connection) -> anyhow::Result<Self> {
        // Create the task pool for processin incoming requests.
        let (sender, receiver) = crossbeam_channel::unbounded();
        let task_pool = TaskPool::with_num_threads(sender, 4)?;
        let task_pool_handle = TaskPoolHandle::new(receiver, task_pool);

        Ok(Server {
            connection,
            req_queue: Default::default(),
            task_pool_handle,
            document_manager: Default::default(),
            diagnostics_manager: Default::default(),
            analysis: Analysis::new(),
        })
    }

    pub(crate) fn snapshot(&self) -> ServerSnapshot {
        ServerSnapshot {
            analysis_snapshot: self.analysis.snapshot(),
            document_manager: Arc::clone(&self.document_manager),
        }
    }

    pub(crate) fn process_changes(&mut self) -> bool {
        let mut change = Change::default();
        let mut document_manager = self.document_manager.write();
        let (has_opened_or_closed_documents, changed_file_ids) = document_manager.take_changes();

        if changed_file_ids.is_empty() {
            return has_opened_or_closed_documents;
        }

        for file_id in changed_file_ids {
            let contents = document_manager
                .contents(file_id)
                .map(|contents| contents.to_string())
                .unwrap_or_else(|| String::new());
            change.add_file(file_id, contents);
        }

        drop(document_manager);

        // Apply the change to our analyzer. This will cancel any affected active Salsa operations.
        self.analysis.apply_change(change);
        true
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
}
