use std::sync::{mpsc, Arc, Mutex};
use std::thread;

mod worker;
use worker::Worker;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        let (sender, receiver) = mpsc::channel();
        let receiver_poitner = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(count);

        for i in 0..count {
            workers.push(Worker::new(i, Arc::clone(&receiver_poitner)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
