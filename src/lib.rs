use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;

pub struct Scheduler {
    max_concurrent: usize,
    running: usize,
    pending: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>,
}

impl Scheduler {
    pub fn new(max_concurrent: usize) -> Self {
        Scheduler {
            max_concurrent,
            running: 0,
            pending: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn schedule<F>(&mut self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let pending = Arc::clone(&self.pending);
        let mut pending_lock = pending.lock().unwrap();
        pending_lock.push_back(Box::new(task));

        println!("Scheduled a new task. Total pending tasks: {}", pending_lock.len());
        self.execute_tasks();
    }

    fn execute_tasks(&mut self) {
        while self.running < self.max_concurrent {
            let mut pending_lock = self.pending.lock().unwrap();
            if pending_lock.is_empty() {
                println!("No pending tasks to execute.");
                break;
            }

            let tasks_to_run = if self.running == self.max_concurrent {
                println!("Max concurrent tasks reached: {}", self.running);
                break;
            } else if self.running == self.max_concurrent - 2 {
                println!("Running tasks: {}, picking 2 from pending.", self.running);
                2
            } else {
                println!("Running tasks: {}, picking 1 from pending.", self.running);
                1
            };

            for _ in 0..tasks_to_run {
                if let Some(task) = pending_lock.pop_front() {
                    self.running += 1;
                    println!("Executing task. Running tasks: {}", self.running);
                    let scheduler = Arc::new(Mutex::new(self.clone())); // Clone the scheduler
                    let scheduler_clone = Arc::clone(&scheduler);

                    thread::spawn(move || {
                        task();
                        let mut scheduler = scheduler_clone.lock().unwrap();
                        scheduler.running -= 1;
                        println!("Task completed. Running tasks: {}", scheduler.running);
                        scheduler.execute_tasks();
                    });
                }
            }
        }
    }
}

// Implement Clone for Scheduler
impl Clone for Scheduler {
    fn clone(&self) -> Self {
        Scheduler {
            max_concurrent: self.max_concurrent,
            running: self.running,
            pending: Arc::clone(&self.pending),
        }
    }
}
