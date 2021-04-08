use std::env;
mod huffman;
mod files;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file_contents = files::read_file(filename);

    let root_node = huffman::get_tree(&file_contents);

    println!("Read file {}\nHuffman Tree Root Freq: {}", filename, root_node.freq);
}
