use bitvec::prelude::*;
use files::reading::*;
use files::writing::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::time::{SystemTime};
use huffman::structure::*;

mod files;
mod huffman;


fn main() {
    println!("cxmprs v1.0.0\n");
    let prog_start = SystemTime::now();

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let (new_filename, size) = compress(file_path);

    let init_size = File::open(file_path).unwrap().metadata().unwrap().len();
    let compression: f32 = 100.0 - (size as f32 / init_size as f32) * 100.0;

    println!(
        "{} -> {}\n{}B -> {}B (Compression: {}%)",
        file_path,
        new_filename,
        init_size, 
        size,
        compression.round()
    );

    println!("Compressed\nDecompressing...");

    decompress(&new_filename);

    match SystemTime::now().duration_since(prog_start) {
        Ok(n) => println!("Took {:.3} seconds.", n.as_secs_f32()),
        _ => {}
    }
}


fn compress(path: &String) -> (String, u64) {
    let file_contents = read_string(path);

    let count_table = gen_count_table(&file_contents);

    let root_node = gen_tree(&count_table);
    let mut codes_table: CodeTable = HashMap::new();

    assign_codes(&root_node, &mut codes_table, bitvec![LocalBits, u8;]);

    // Counting compression metrics
    return write(path, &file_contents, &codes_table, &count_table);
}


fn decompress(path: &String) {
    let bytes = decompress(path);
}