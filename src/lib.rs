use std::{thread::{self, JoinHandle}, sync::{mpsc::{self, Receiver}, Arc, Mutex}};

pub struct Threadpool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Threadpool {
    pub fn new (size:usize) -> Threadpool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for thread_id in 0..size {
            //create threads
            let worker = Worker::new(thread_id, Arc::clone(&receiver));
            threads.push(worker);
        }
        Threadpool { workers: threads , sender: Some(sender) }
    }

    pub fn execute<F> (&self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);

            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(worker_id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move||loop {
            let msg = receiver.lock().unwrap().recv();

            match msg {
                Ok(job) => { println!("Worker (id:{worker_id}) executing job");
                job(); }
                Err (_) => { println!("Worker (id:{worker_id}) is shuting down");
                break; }
            }
        });
        Worker {id: worker_id, handle: Some(thread),}
    }
}