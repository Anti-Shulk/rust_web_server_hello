use std::{thread, sync::{mpsc, Arc, Mutex}, error::Error, num::{ParseIntError, IntErrorKind}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// The size is the number of threads in the pool.
    /// 
    /// # Error
    /// This function will return an Error if `size` is 0.
    pub fn new(size: usize) -> Result<ThreadPool, IntErrorKind> {
        if size <= 0 { return Err(IntErrorKind::Zero); }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        

        Ok(ThreadPool { workers, sender: Some(sender) })
    }

    pub fn execute<F>(&self, function: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(function);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        self.workers.iter_mut().for_each(|worker| {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        })
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {} got job; executing.", id);
                    job();
                },
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                },
            }
        });

        Worker { id, thread: Some(thread) }
    }
}