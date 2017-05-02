use std::io::prelude::*;
use std::io;
use std::process;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LIMIT: RwLock<u32> = RwLock::new(0);
}

fn main() {
    // input
    let mut input = String::new();
    print!("Enter the largest number for the sieve > ");
    io::stdout().flush().ok().expect("flush failed");
	io::stdin().read_line(&mut input).expect("read_line failed");
	{
		let mut limit = LIMIT.write().unwrap();
		*limit = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("invalid input");
				process::exit(1);
			}
		};
	}

	// take a read lock so that limit can not be modified
	let limit = LIMIT.read().unwrap();
	println!("Generating prime numbers up to {}", *limit);

	// start sieve
	let (tx, handle) = sieve_start(1);
	
	// send in initial values
	for i in 1..(*limit) {
		let i = i+1;
		tx.send(i).unwrap();
	}
	drop(tx);

	// wait for completion
	handle.join().unwrap();
}

fn sieve_start(divisor: u32) -> (Sender<u32>, JoinHandle<()>) {
	let (tx, rx) = mpsc::channel();

	let handle = thread::spawn(move || {
		sieve(divisor, rx);
	});

	(tx, handle)
}

fn sieve(divisor: u32, feed: Receiver<u32>) {
	let mut next_divisor = 0;

	loop {
		let value = match feed.recv() {
			Ok(v) => v,
			Err(_) => {
				// reached limit
				break;
			},
		};
		
		if value % divisor != 0 || divisor == 1 {
			next_divisor = value;
			break;
		}
	}

	if next_divisor == 0 {
		return;
	}

	println!("{:?}", next_divisor);

	let handle = {
		// start next stage
		let (tx, handle) = sieve_start(next_divisor);

		// finish going through feed
		for value in feed {
			if value % divisor == 0 && divisor != 1 {
				continue;
			}

			// pass to next stage
			//println!("Passing {} from divisor {}", value, divisor);
			tx.send(value).unwrap();
		}

		drop(tx);

		handle
	};
	
	// wait for sieve to finish
	handle.join().unwrap();
}
