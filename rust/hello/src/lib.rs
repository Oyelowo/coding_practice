use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        (&self.workers).into_iter().for_each(|_| {
            self.sender.send(Message::Terminate).unwrap();
        });

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            /*     match worker.thread.take() {
                Some(th) => th.join().unwrap(),
                None => ()
            } */
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    ///  # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        /*
        our ThreadPool creates a new vector that can hold size items. We haven’t used the with_capacity function in this book yet, which performs the same task as Vec::new but with an important difference: it preallocates space in the vector. Because we know we need to store size elements in the vector, doing this allocation up front is slightly more efficient than using Vec::new, which resizes itself as elements are inserted.
         */
        let mut workers = Vec::with_capacity(size);

        /*   for id in 0..size {
            // Create some threads and store them in the vector.
            workers.push(Worker::new(id));
        } */

        let receiver = Arc::new(Mutex::new(receiver));
        (0..size).for_each(|id| {
            // Create some threads and store them in the vector.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        });

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Failed to acquire the mutex")
                .recv()
                .expect("Something went wrong with receiving the job from the channel. Might the sending channel be shutdown?");

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
