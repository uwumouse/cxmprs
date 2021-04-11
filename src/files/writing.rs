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
    let new_filename = get_new_filename(init_filename);
    let mut out_file = File::create(new_filename).unwrap();
    write_header(count_table, &mut out_file);
    let out_data = gen_out_data(init_contents, codes_table);
    let out_data = to_u8_vec(out_data);

    write_compressed(out_data, &mut out_file);

    return out_file.metadata().unwrap().len();
}

fn get_new_filename(init_filename: &String) -> String {
    let mut file = String::from(
        Path::new(init_filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    file.push_str(".cxmp");

    return file;
}

fn write_header(count_table: &CharMap, file: &mut File) {
    let sep = String::from("sc"); // stands for "seperator character"
    let sep_bytes = sep.as_bytes();

    for (ch, freq) in count_table.iter() {
        let mut char_to_write = ch.clone();
        if char_to_write.eq("\n") {
            char_to_write = String::from("nl");
        }
        match file.write_all(
            &[
                char_to_write.as_bytes(),
                sep_bytes,
                &freq.to_ne_bytes(),
                sep_bytes,
            ]
            .concat(),
        ) {
            Ok(_) => {}
            _ => {
                println!("ERR: Failed to write header for the file");
                std::process::exit(1);
            }
        }
    }
}

fn write_compressed(data: Vec<u8>, file: &mut File) {
    match file.write_all(&[String::from("dd").as_bytes(), &data[..]].concat()) {
        Ok(_) => {}
        _ => {
            println!("ERR: Failed to write compressed data to the file");
            std::process::exit(1);
        }
    };
}
