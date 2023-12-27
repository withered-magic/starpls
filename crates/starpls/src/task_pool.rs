use crossbeam_channel::{Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};

pub(crate) struct TaskPool<T> {
    sender: Sender<T>,
    inner: ThreadPool,
}

impl<T> TaskPool<T> {
    pub(crate) fn with_num_threads(
        sender: Sender<T>,
        num_threads: usize,
    ) -> anyhow::Result<TaskPool<T>> {
        let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build()?;
        Ok(TaskPool {
            sender,
            inner: thread_pool,
        })
    }

    fn spawn<F>(&self, f: F)
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.inner.spawn({
            let sender = self.sender.clone();
            move || sender.send(f()).unwrap()
        })
    }
}

pub(crate) struct TaskPoolHandle<T> {
    pub(crate) receiver: Receiver<T>,
    pool: TaskPool<T>,
}

impl<T> TaskPoolHandle<T> {
    pub(crate) fn new(receiver: Receiver<T>, pool: TaskPool<T>) -> Self {
        Self { receiver, pool }
    }

    pub(crate) fn spawn<F>(&self, f: F)
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.pool.spawn(f)
    }
}
