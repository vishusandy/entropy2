use std::io::prelude::*;
use std::fs::File;
use std::env;

// extern crate time;
// use time::PreciseTime;

// use std::time::Instant;
use std::time::{Duration, SystemTime, Instant};

fn main() {
	//let mut f = io::BufReader::new(try!(fs::File::open("input.txt")));
	//let mut f = File::open("sample.txt").expect("Unable to open file");
	
	//let mut f = io::BufReader::new(try!(fs::File::open("sample.txt")));
	
	// let mut f = File::open("sample.txt")?;
	//let mut f = File::open("sample.txt")?;
	// let mut f = try!(fs::File::open("sample.txt"));
	
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		println!("No file specified.");
	} else {
		// let beginning = PreciseTime::now();
		let beginning = Instant::now();
		for a in 1..args.len() {
			let arg = &args[a];
			let f = File::open(arg).expect("Unable to open file");
			
			// let start = PreciseTime::now();
			let start = Instant::now();
			
			let mut t = 0;
			let mut entropy: f32 = 0.0;
			let mut histogram = [0; 256];
			
			for b in f.bytes() {
				
				histogram.get_mut(b.unwrap() as usize).map(|v| *v += 1);
				t += 1;
			}
			for i in 0..256 {
				if histogram[i] > 0 {
					let ratio = (histogram[i] as f32 / t as f32) as f32;
					entropy -= (ratio * ratio.log2()) as f32;
				}
			}
			// let end = (start.as_secs() * 1_000) + (start.subsec_nanos() / 1_000_000) as u64;
			let end = start.elapsed();
			println!("File {} entropy: {}% in {:?} seconds", a, entropy/8.0*100.0, end);
			// println!("File {} entropy: {}% in {} seconds", a, entropy/8.0*100.0, start.to(PreciseTime::now()));
		}
		// let ending = (beginning.as_secs() * 1_000) + (beginning.subsec_nanos() / 1_000_000) as u64;
		let ending = beginning.elapsed();
		println!("Exec time: {:?}", ending);
		// println!("Exec time: {}", beginning.to(PreciseTime::now())*1);
	}
	/*
	let mut hist: [u8] = [0; 256];
	let mut t = 0;
	
	for b in f.bytes() {
		let idx: u8 = b;
		hist[idx] += 1;
		t += 1;
		//print!("{}", b.unwrap());st
	}
	
	
	let mut entropy = 0.0;
	for i in 0..256 {
		if hist[i] > 0 {
			let p: f64 = hist[i] / t;
			entropy += -p * p.log2();
		}
	}
	
	println!("Entropy: {}", entropy/8.0);
	*/
}
