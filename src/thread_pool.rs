use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Tạo một ThreadPool mới.
    ///
    /// Kích thước là số luồng trong pool.
    ///
    /// # Panics
    ///
    /// Hàm `new` sẽ báo lỗi nếu kích thước bằng không.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn execute() -> ThreadPool {
        ThreadPool::new(4)
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
