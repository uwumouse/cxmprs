mod huffman;
mod files;

use bitvec::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use huffman::encoding::*;
use files::reading::*;
use files::writing::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file_contents = read_file(filename);


    let count_table = gen_count_table(&file_contents);
    let root_node = gen_tree(&count_table);

    
    let mut codes_table: CodeTable = HashMap::new();
    let code: U8BitVec  = bitvec![LocalBits, u8;];

    assign_codes(&root_node, &mut codes_table, code);


    let size = write(filename, &file_contents, &codes_table, &count_table);
    let init_size = File::open(filename).unwrap().metadata().unwrap().len();

    let compression: f32 = 100.0 - (size as f32 / init_size as f32) * 100.0;

    println!("Compressed.\n{}b -> {}b (Compression: {}%)", init_size, size, compression.round());
}
