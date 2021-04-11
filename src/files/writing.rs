use crate::huffman::encoding::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn write(
    init_filename: &String,
    init_contents: &String,
    codes_table: &CodeTable,
    count_table: &CharMap,
) -> u64 {
    let new_filename = change_ext(init_filename);
    let mut out_file = File::create(new_filename).unwrap();

    let out_data = gen_out_data(init_contents, codes_table);

    write_header(count_table, &mut out_file);
    write_compressed(out_data, &mut out_file);

    return out_file.metadata().unwrap().len();
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
                &[29] // GS (Group separator) separates char and it freq from other ones in header
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
            &[31], // US (unit separator), separates header & data
            &data[..],
            &[3], // EOT (End Of Text)
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
