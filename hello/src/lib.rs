use std::thread;
pub struct ThreadPool{
  threads: Vec<thread::JoinHandle<()>>,
};

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // alternatively could return a rResult
        let mut threads = Vec::with_capacity(size)
        for _ in 0..size{}
        ThreadPool
    }

    // external signature
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,
    // {
    // }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
