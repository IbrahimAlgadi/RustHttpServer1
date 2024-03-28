use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// struct Job;

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciver) = mpsc::channel();

        let reciver = Arc::new(Mutex::new(reciver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some workers and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&reciver)));

        }
        ThreadPool { 
            workers, 
            sender: Some(sender) 
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Dropping sender closes the channel, which indicates no more messages will be sent.
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing..");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
            // reciver;
        });
        Worker {
            id, 
            thread: Some(thread),
        }
    }
}