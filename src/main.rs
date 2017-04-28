use std::io::prelude::*;
use std::io;
use std::process;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    // input
    let mut limit = String::new();
    print!("Enter the largest number for the sieve > ");
    io::stdout().flush().ok().expect("flush failed");
	io::stdin().read_line(&mut limit).expect("read_line failed");
	let limit: u32 = match limit.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("invalid input");
			process::exit(1);
		}
	};
	println!("Generating prime numbers up to {}", limit);

	// start sieve
	let handle = {
		let (tx, handle) = sieve_start(2);

		for i in 0..limit {
			let i = i+1;
			tx.send(i).unwrap();
		}

		handle
	};

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
	for inp in feed {
		if inp % divisor != 0 {
			println!("{:?}", inp);
		}
	}
}
