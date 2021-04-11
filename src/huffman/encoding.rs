use crate::huffman::structures::Node;
use bitvec::prelude::*;
use std::collections::HashMap;

pub type U8BitVec = BitVec<LocalBits, u8>;
pub type CodeTable = HashMap<char, U8BitVec>;
pub type CharMap = HashMap<char, usize>;

pub fn gen_tree(count_table: &CharMap) -> Box<Node> {
    // Get list of leaf nodes based on character map
    let nodes: Vec<Box<Node>> = count_table
        .into_iter()
        .map(|(ch, freq)| Box::new(Node::new(*freq, Some(*ch))))
        .collect();

    // Create single tree
    return nodes_into_tree(nodes);
}

// Returns a hashmap with every character and it's frequency
pub fn gen_count_table(content: &String) -> CharMap {
    let mut count_table: CharMap = HashMap::new();
    // List of every char in string
    let chars: Vec<char> = content.chars().collect();
    // Counting occurences of every character in string
    for ch in chars {
        *count_table.entry(ch).or_insert(0) += 1;
    }

    return count_table;
}

// Goes through tree untill find leaf node and saves code for this char ino codes table
pub fn assign_codes(node: &Box<Node>, codes_table: &mut CodeTable, code: U8BitVec) {
    // Only leaf nodes have values
    if let Some(val) = &node.val {
        codes_table.insert(val.clone(), code);
    } else {
        // Go to left and try to find leaf node there
        if let Some(ref left) = node.left {
            let mut left_code = code.clone();
            left_code.extend_from_raw_slice(&[0]);
            // But add 0 since turned to left
            assign_codes(left, codes_table, left_code.clone());
        }
        // Same as left, but add 1 since went to the right
        if let Some(ref right) = node.right {
            let mut right_code = code.clone();
            right_code.extend_from_raw_slice(&[1]);
            assign_codes(right, codes_table, right_code.clone());
        }
    }
}

fn nodes_into_tree(mut nodes: Vec<Box<Node>>) -> Box<Node> {
    while nodes.len() > 1 {
        // Descending sorting
        nodes.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        // Get two node with least frequency
        let n1 = nodes.pop().unwrap();
        let n2 = nodes.pop().unwrap();
        let mut parent = Box::new(Node::new(n1.freq + n2.freq, None));
        parent.right = Some(n1);
        parent.left = Some(n2);

        nodes.push(parent);
    }
    return nodes.pop().unwrap();
}

// Generate final vector of bytes
pub fn gen_out_data(file_contents: &String, codes_table: &CodeTable) -> Vec<u8> {
    let mut out_bits: U8BitVec = bitvec![LocalBits, u8;];

    let content_chars: Vec<char> = file_contents.chars().collect();

    for ch in content_chars {
        add_code(&mut out_bits, codes_table, ch);
    }

    return to_bytes_vec(out_bits);
}

// Finds character in codes table and adds code to the out bits
fn add_code(out_bits: &mut U8BitVec, codes_table: &CodeTable, char_to_find: char) {
    for (ch, code) in codes_table.iter() {
        if char_to_find == *ch {
            out_bits.extend_from_bitslice(code);
        }
    }
}

// Generates bytes from plain bits vector
fn to_bytes_vec(data_bits: U8BitVec) -> Vec<u8> {
    let data_vec = data_bits.into_vec();
    let len = data_vec.len();
    let mut bytes: Vec<u8> = vec![];
    let base: u8 = 2; 
    // Save base to use .pow func
    // Current part if bits
    // step for this will be 8
    let mut i = 0;

    while i < len {
        let mut number: u8 = 0; // Number current byte will be represent
        let remainer = if len - i > 8 { 8 } else { len - i }; // How many bits are avaliable to put in byte

        for bit_index in 0..remainer {
            number += base.pow(bit_index as u32) * data_vec[i + bit_index];
        }

        bytes.push(number);

        i += 8;
    }

    return bytes;
}
