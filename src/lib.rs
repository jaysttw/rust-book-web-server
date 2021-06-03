use std::{thread::Thread, usize};
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job {

}

impl ThreadPool {
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    pub fn new(size: usize) -> ThreadPool {
        /// Create a new ThreadPool.
        ///
        /// The size is the number of threads in the pool.
        ///
        /// # Panics
        ///
        /// The `new` function will panic if the size is zero.
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        
        let mut workers = Vec::with_capacity(size); // `with_capacity` pre-allocates fixed size

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce() + Send + 'static,
    {
        // Should have a similar interface to `thread::spawn`.
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}