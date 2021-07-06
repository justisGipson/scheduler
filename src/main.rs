extern crate job_scheduler;

use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

fn main() {
	let mut scheduler = JobScheduler::new();

	// adding a task to scheduler to execute it every 2 minutes
	scheduler.add(Job::new("1/2 * * * * *".parse().unwrap(),|| {
		println!("Get executed every 2 seconds");
	}));

	scheduler.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
		println!("Get executed every 10 seconds");
	}));

	loop {
		scheduler.tick();
		std::thread::sleep(Duration::from_millis(100));
	}
}
