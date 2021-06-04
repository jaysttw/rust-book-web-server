use std::{sync::{Mutex, mpsc, Arc}, usize};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size); // `with_capacity` pre-allocates fixed size

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce() + Send + 'static,
    {
        // Should have a similar interface to `thread::spawn`.
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap(); // `unwrap` is used because failure case cannot happen.
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap(); // possibly rewrite to produce errors instead of panic.

            println!("Worker {} got a job; executing.", id);
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a new job; executing...", id)
                }
                Message::Terminate => {
                    println!("Worker {} received an order to terminate.", id)
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}