use std::fs;
use std::process;
use std::io::ErrorKind;

pub fn read_file(filename: &String) -> String {
    let file_contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("File {} not found", filename);
                process::exit(1);
            },
            _ => {
                println!("Unknown error occured:\n{}", err);
                process::exit(1);
            }
        }
    };

    return file_contents;
}