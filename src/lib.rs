use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// count is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(count: usize) -> Self {
        assert!(count > 0);

        let mut workers = Vec::with_capacity(count);

        for i in 0..count {
            workers.push(Worker::new(i));
        }

        println!("creating a ThreadPool with count {count}");
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("executing thread");
        f();
    }
}

struct Worker {
    id: usize,
    worker_thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        let worker_thread = thread::spawn(|| {});
        Worker { id, worker_thread }
    }
}
