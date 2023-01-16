// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut previous_jobs_completed = 0;

    for _ in 0..10 {
        let shared_status = status.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(4000));
            // TODO: You must take an action before you update a shared value
            let mut job_status = shared_status.lock().unwrap();
            job_status.jobs_completed += 1;
        });
    }

    loop {
        let shared_status = status.clone();
        let job_status = shared_status.lock().unwrap();
        if previous_jobs_completed != job_status.jobs_completed {
            println!("jobs completed {}", job_status.jobs_completed);
        }

        if job_status.jobs_completed == 10 {
            break;
        }

        previous_jobs_completed = job_status.jobs_completed;
    }
}
