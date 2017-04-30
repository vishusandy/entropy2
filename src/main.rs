use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::io::BufReader;
use std::time::Instant;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		println!("No file specified.");
	} else {
		// let beginning = PreciseTime::now();
		let beginning = Instant::now();
		for a in 1..args.len() {
			let arg = &args[a];
			let f = File::open(arg).expect("Unable to open file");
			let r = BufReader::new(f);

			let start = Instant::now();
			let mut t = 0;
			let mut entropy: f32 = 0.0;
			let mut histogram = [0; 256];
			
			for b in r.bytes() {
				histogram.get_mut(b.unwrap() as usize).map(|v| *v += 1);
				t += 1;
			}
			for i in 0..256 {
				if histogram[i] > 0 {
					let ratio = (histogram[i] as f32 / t as f32) as f32;
					entropy -= (ratio * ratio.log2()) as f32;
				}
			}
			let end = start.elapsed();
			println!("File {} entropy: {}% in {:?} seconds", a, entropy/8.0*100.0, end);
		}
		let ending = beginning.elapsed();
		println!("Exec time: {:?}", ending);
	}
}
