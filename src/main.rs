use std::env;
mod huffman;
mod files;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file_contents = files::read_file(filename);

    let mut root_node = huffman::generate_tree(&file_contents);

    huffman::tree_to_codes(&mut root_node);
}
