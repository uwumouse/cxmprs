use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use crate::huffman::structure::*;
use crate::huffman::encoding::*;
use crate::control_codes::*;


pub fn write(
    init_filename: &String,
    init_contents: &String,
    codes_table: &CodesTable,
    charmap: &CharMap,
) -> (String, u64) {
    let new_filename = change_ext(init_filename);
    let mut out_file = File::create(&new_filename).unwrap();

    let out_data = gen_out_data(init_contents, codes_table);

    write_header(charmap, &mut out_file);
    write_compressed(out_data, &mut out_file);

    return (new_filename.clone(), out_file.metadata().unwrap().len());
}

// Replaces original file extension with new one
// Return just a file name, without actual path
fn change_ext(init_path: &String) -> String {
    let mut filename = String::from(
        Path::new(init_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    filename.push_str(".cxmp");

    return filename;
}

// Writes data about characters and their frequency
fn write_header(charmap: &CharMap, file: &mut File) {
    for (ch, freq) in charmap.iter() {
        let mut freq_bytes = vec![];

        // Removing bytes with the value from number byte-representation 
        for i in freq.to_ne_bytes().iter() {
            if i > &0 {
                freq_bytes.push(i.clone());
            }
        }
        
        match file.write_all(
            &[
                &[*ch as u8],
                &freq_bytes[..],
                &[GS]
            ].concat()
        ) {
            Ok(_) => {}
            _ => {
                println!("ERR: Failed to write header for the file");
                std::process::exit(1);
            }
        }
    }
}

// Write compressed data to the file
fn write_compressed(data: Vec<u8>, file: &mut File) {
    match file.write_all(
        &[
            &[US],
            &data[..],
        ]
        .concat(),
    ) {
        Ok(_) => {}
        _ => {
            println!("ERR: Failed to write compressed data to the file");
            std::process::exit(1);
        }
    };
}
