use std::{thread::Thread, usize};

pub struct ThreadPool;

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
        assert!(size > 0;
        )
        ThreadPool
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce() + Send + 'static,
    {
        // Should have a similar interface to `thread::spawn`.
    }
}