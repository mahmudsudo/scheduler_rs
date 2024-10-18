// Import the Scheduler struct from your library
use scheduler_rs::Scheduler; // Adjust the path if necessary
use std::sync::{Arc, Mutex}; // Import Arc and Mutex
use std::thread; // Import the thread module
use std::time::Duration; // Import Duration for sleep

fn main() {
    let scheduler = Arc::new(Mutex::new(Scheduler::new(4))); // Wrap in Arc and Mutex
    let mut handles = vec![]; // Vector to hold thread handles

    for i in 0..10 {
        let scheduler_clone = Arc::clone(&scheduler); // Clone the Arc for each thread
        let task = move || {
            println!("Task {} is running", i);
            thread::sleep(Duration::from_secs(1)); // Simulate work
        };

        // Schedule the task and store the handle
        let handle = thread::spawn(move || {
            let mut scheduler = scheduler_clone.lock().unwrap(); // Lock the mutex
            scheduler.schedule(task);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Give some time for tasks to complete
    thread::sleep(Duration::from_secs(1));
}
