use std::time::Duration;

use crossbeam_channel::{RecvError, RecvTimeoutError, Sender};
use rustc_hash::FxHashSet;
use starpls_common::FileId;

use crate::event_loop::Task;

pub(crate) struct AnalysisDebouncer {
    pub(crate) sender: Sender<Vec<FileId>>,
}

impl AnalysisDebouncer {
    pub(crate) fn new(duration: Duration, sink: Sender<Task>) -> Self {
        let (source_tx, source_rx) = crossbeam_channel::unbounded::<Vec<FileId>>();

        std::thread::spawn(move || {
            let mut active = false;
            let mut pending_file_ids: FxHashSet<FileId> = FxHashSet::default();
            loop {
                if active {
                    match source_rx.recv_timeout(duration) {
                        Ok(file_ids) => pending_file_ids.extend(file_ids.into_iter()),
                        Err(RecvTimeoutError::Disconnected) => break,
                        Err(RecvTimeoutError::Timeout) => {
                            sink.send(Task::AnalysisRequested(pending_file_ids.drain().collect()))
                                .unwrap();
                            active = false;
                        }
                    }
                } else {
                    match source_rx.recv() {
                        Ok(file_ids) => {
                            active = true;
                            pending_file_ids.extend(file_ids.into_iter());
                        }
                        Err(RecvError) => break,
                    }
                }
            }
        });

        Self { sender: source_tx }
    }
}
