use bitvec::prelude::*;
use files::reading::*;
use files::writing::*;
use huffman::encoding::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::time::{SystemTime};
use std::fs;

mod files;
mod huffman;


fn main() {
    println!("cxmprs v1.0.0\n");
    let prog_start = SystemTime::now();

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file_contents = read_file(filename);

    let count_table = gen_count_table(&file_contents);
    let root_node = gen_tree(&count_table);
    let mut codes_table: CodeTable = HashMap::new();

    assign_codes(&root_node, &mut codes_table, bitvec![LocalBits, u8;]);


    // Counting compression metrics
    let (size, new_filename) = write(filename, &file_contents, &codes_table, &count_table);

    let mut prog_duration: std::time::Duration = std::time::Duration::new(1, 1);

    match SystemTime::now().duration_since(prog_start) {
        Ok(n) => prog_duration = n,
        _ => {}
    }

    let init_size = File::open(filename).unwrap().metadata().unwrap().len();
    let compression: f32 = 100.0 - (size as f32 / init_size as f32) * 100.0;

    println!(
        "{} -> {}\n{}B -> {}B (Compression: {}%)",
        filename,
        new_filename,
        init_size, 
        size,
        compression.round()
    );
    println!("Took {:.3} seconds.", prog_duration.as_secs_f32());
}
