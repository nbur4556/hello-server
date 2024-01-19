use crate::thread_pool::Job;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
    id: usize,
    worker_thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let worker_thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, worker_thread }
    }
}
