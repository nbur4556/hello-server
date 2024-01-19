use crate::thread_pool::Job;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
    pub id: usize,
    pub worker_thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let worker_thread = Some(thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        }));

        Worker { id, worker_thread }
    }
}
