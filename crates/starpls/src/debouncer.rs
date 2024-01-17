use crate::event_loop::Task;
use crossbeam_channel::{RecvError, RecvTimeoutError, Sender};
use std::time::Duration;

pub(crate) struct AnalysisDebouncer {
    pub(crate) sender: Sender<()>,
}

impl AnalysisDebouncer {
    pub(crate) fn new(duration: Duration, sink: Sender<Task>) -> Self {
        let (source_tx, source_rx) = crossbeam_channel::unbounded();

        std::thread::spawn(move || {
            let mut active = false;
            loop {
                if active {
                    match source_rx.recv_timeout(duration) {
                        Ok(_) => {}
                        Err(RecvTimeoutError::Disconnected) => break,
                        Err(RecvTimeoutError::Timeout) => {
                            sink.send(Task::AnalysisRequested).unwrap();
                            active = false;
                        }
                    }
                } else {
                    match source_rx.recv() {
                        Ok(_) => active = true,
                        Err(RecvError) => break,
                    }
                }
            }
        });

        Self { sender: source_tx }
    }
}
