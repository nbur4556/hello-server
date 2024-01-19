use std::sync::{mpsc, Arc, Mutex};

mod worker;
use worker::Worker;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.worker_thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
