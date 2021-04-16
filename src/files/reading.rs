use std::fs;
use std::io::ErrorKind;
use std::process;

pub fn read_string(path: &String) -> String {
	let file_contents = match fs::read_to_string(path) {
		Ok(c) => c,
		Err(err) => match err.kind() {
			ErrorKind::NotFound => {
				println!("File {} not found", path);
				process::exit(1);
			}
			ErrorKind::InvalidData => {
				println!("Only UTF-8 text format are readable.");
				process::exit(1);
			}
			_ => {
				println!("Unknown error occured:\n{}", err);
				process::exit(1);
			}
		},
	};

	return file_contents;
}

pub fn read_compressed(path: &String) -> Vec<u8> {
	return match fs::read(path) {
		Ok(c) => c,
		Err(err) => match err.kind() {
			ErrorKind::NotFound => {
				println!("File {} not found", path);
				process::exit(1);
			}
			_ => {
				println!("Unknown error occured:\n{}", err);
				process::exit(1);
			}
		},
	};
}