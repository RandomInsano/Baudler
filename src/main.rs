// Delay sending data similarly to using a dial-up modem.

use std::{
	io,
	thread,
	time
};
use std::io::prelude::*;

// Delay per byte to emulate modem
const BAUD_300: u64 = 27; // 26.666
const BAUD_600: u64 = 13; // 13.333
const BAUD_1200: u64 = 7; // 6.666

fn main() {
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	let stdout = io::stdout();
	let mut stdout = stdout.lock();
	let mut buffer = [0; 1024];

	let delay = time::Duration::from_millis(BAUD_1200);

	while let Ok(length) = stdin.read(&mut buffer) {
		if length < 1 {
			println!("Reached end of file");
			return;
		}

		for b in 0 .. length {
			stdout.write(&[buffer[b]]).unwrap();
			stdout.flush();
			thread::sleep(delay);
		}
	}
}
