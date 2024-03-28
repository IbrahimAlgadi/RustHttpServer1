use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some workers and store them in the vector
            workers.push(Worker::new(id, reciver));

        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciver: mpsc::Reciver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            reciver;
        });
        Worker {id, thread}
    }
}