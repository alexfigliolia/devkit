use tokio::{
    runtime::{self, Runtime},
    task::JoinHandle,
};

pub struct ThreadPool {
    pool: Runtime,
}

impl ThreadPool {
    pub fn new(threads_override: Option<usize>, pool_override: Option<Runtime>) -> ThreadPool {
        let threads = threads_override.or(Some(4 as usize)).unwrap();
        let pool = pool_override
            .or(Some(
                runtime::Builder::new_multi_thread()
                    .worker_threads(threads) // Use worker_threads or core_threads depending on version/docs
                    .enable_all()
                    .build()
                    .unwrap(),
            ))
            .unwrap();
        ThreadPool { pool }
    }

    pub fn spawn<T: Send + 'static, F: (Fn() -> T) + 'static + Send>(
        &mut self,
        task: F,
    ) -> JoinHandle<T> {
        return self.pool.block_on(async {
            return tokio::spawn(async move { task() });
        });
    }
}
