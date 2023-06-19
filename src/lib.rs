use std::fs::File;
use std::io::{self, BufRead};

use csv;

use convert::{Converter, HotelRecord, RoomRecord};

mod convert;

pub struct Files {
	pub input: File,
	pub hotels: File,
	pub room_names: File,
	pub output: File,
}

pub fn perform_conversion(files: Files) -> Result<(), ()> {

	let hotels = io::BufReader::new(files.hotels)
		.lines()
		.map(Result::unwrap)
		.map(|s| serde_json::from_str::<HotelRecord>(&s))
		.map(|result| result.expect("Parsing hotel record"));

	let mut room_reader = csv::ReaderBuilder::new()
		.delimiter(b'|')
		.has_headers(false)
		.from_reader(files.room_names);

	let rooms = room_reader
		.deserialize::<RoomRecord>()
		.map(|result| result.expect("Parsing room record"));

	let converter = Converter::with_details(hotels, rooms);

	let mut input_parse_errors: u64 = 0;
	let mut convert_errors: u64 = 0;
	let mut output_ser_errors: u64 = 0;

	let mut input_reader = csv::ReaderBuilder::new()
		.delimiter(b'|')
		.from_reader(files.input);

	let input = input_reader.deserialize()
		.filter_map(|result| {
			match result {
				Ok(record) => Some(record),
				Err(e) => {
					eprintln!("Parsing input record: {}", e);
					input_parse_errors += 1;
					None
				}
			}
		});

	let mut output_writer = csv::WriterBuilder::new()
		.delimiter(b';')
		.has_headers(true)
		.from_writer(files.output);

	let output = input.map(|record| converter.transform(record))
		.filter_map(|result| {
			match result {
				Ok(record) => Some(record),
				Err(e) => {
					eprintln!("Converting record: {}", e);
					convert_errors += 1;
					None
				}
			}
		});

	for record in output {
		match output_writer.serialize(record) {
			Err(e) => {
				eprintln!("Serializing output record: {}", e);
				output_ser_errors += 1;
			},
			_ => (),
		}
	}

	println!("Parsed input file with {} errors", input_parse_errors);
	println!("Converted to output format with {} errors", convert_errors);
	println!("Serialized to output file with {} errors", output_ser_errors);

	output_writer.flush().unwrap();

	match (input_parse_errors, convert_errors, output_ser_errors) {
		(0, 0, 0) => Ok(()),
		_ => Err(()),
	}
}
