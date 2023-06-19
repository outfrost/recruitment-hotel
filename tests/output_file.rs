use std::fs::{DirBuilder, File};
use std::io::Read;

use recruitment_task::Files;

#[test]
fn converts_example() {
	DirBuilder::new()
		.recursive(true)
		.create("out/tests").unwrap();

	let files = Files {
		input: File::open("data/input.csv").unwrap(),
		hotels: File::open("data/hotels.json").unwrap(),
		room_names: File::open("data/room_names.csv").unwrap(),
		output: File::create("out/tests/output.csv").unwrap(),
	};

	assert_eq!(Ok(()), recruitment_task::perform_conversion(files));

	let mut expected_output = String::new();
	let mut actual_output = String::new();

	File::open("data/expected.csv").unwrap()
		.read_to_string(&mut expected_output).unwrap();

	let mut output_file = File::open("out/tests/output.csv").expect("Output file was not created, or is inaccessible");
	output_file.read_to_string(&mut actual_output).expect("Cannot read output file");

	assert_eq!(expected_output, actual_output);
}
