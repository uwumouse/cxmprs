use crate::huffman::structures::*;
use crate::huffman::encoding::*;
use crate::files::control_codes::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn write(
    init_filename: &String,
    init_contents: &String,
    codes_table: &CodeTable,
    count_table: &CharMap,
) -> (u64, String) {
    let new_filename = change_ext(init_filename);
    let mut out_file = File::create(&new_filename).unwrap();

    let out_data = gen_out_data(init_contents, codes_table);

    write_header(count_table, &mut out_file);
    write_compressed(out_data, &mut out_file);

    return (out_file.metadata().unwrap().len(), new_filename.clone());
}

// Replaces original file extension with new one
fn change_ext(init_filename: &String) -> String {
    let mut filename = String::from(
        Path::new(init_filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    filename.push_str(".cxmp");

    return filename;
}

// Writes data about characters and their frequency
fn write_header(count_table: &CharMap, file: &mut File) {
    for (ch, freq) in count_table.iter() {
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
