use std::io::prelude::*;
use std::io;
use std::process;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;


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

	// boot
	let (tx, rx) = mpsc::channel();
	let handles: Vec<JoinHandle<_>> = (0..limit).map(|p| {
	    let tx = tx.clone();

	    thread::spawn(move || {
	        tx.send(p).unwrap();
	    })
	}).collect();


	for _ in 0..limit {
		println!("{:?}", rx.recv().unwrap());
	}

	for h in handles {
        h.join().unwrap();
    }
}
