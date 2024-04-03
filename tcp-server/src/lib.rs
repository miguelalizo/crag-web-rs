use std::error;
use std::fmt;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for PoolCreationError { }   


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threadsa in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError::ZeroSize);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));


        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job: Job = Box::new(f);
        self.sender.send(job).expect("sender error");
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // if OS cannot create a new thread, thread::spawn will panic
        // TODO: Change to thread::Builder which returns Result
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} received job. Executing.");

            job();
        } 
        
        );

        Worker { id, thread }
    }
}