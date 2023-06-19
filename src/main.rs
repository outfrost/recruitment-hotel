use std::{env, fs::File, process};

use recruitment_hotel::Files;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 5 {
		eprintln!("Not enough arguments.");
		eprintln!("USAGE:");
		eprintln!("    {} <input_file> <hotels_file> <room_names_file> <output_file>", &args[0]);
		process::exit(2);
	}

	let files = Files {
		input: File::open(&args[1]).unwrap(),
		hotels: File::open(&args[2]).unwrap(),
		room_names: File::open(&args[3]).unwrap(),
		output: File::create(&args[4]).unwrap(),
	};

	match recruitment_hotel::perform_conversion(files) {
		Ok(()) => (),
		Err(()) => {
			process::exit(1);
		},
	}
}
