use bitvec::prelude::*;
use std::collections::HashMap;
use crate::huffman::structure::Node;

pub type U8BitVec = BitVec<LocalBits, u8>;
pub type CodeTable = HashMap<String, U8BitVec>;
pub type CharMap = HashMap<String, usize>;

pub fn gen_tree(count_table: &CharMap) -> Box<Node> {
    let nodes: Vec<Box<Node>> = count_table
        .into_iter()
        .map(|(ch, freq)| Box::new(Node::new(*freq, Some(ch.clone()))))
        .collect();

    return nodes_into_tree(nodes);
}
pub fn gen_count_table(content: &String) -> CharMap {
    let mut count_table: CharMap = HashMap::new();
    // List of every char in string
    let chars: Vec<char> = content.chars().collect();
    // Counting occurences of every character in string
    for ch in chars {
        *count_table.entry(String::from(ch)).or_insert(0) += 1;
    }

    *count_table.entry(String::from("eof")).or_insert(0) += 1;

    return count_table
}
pub fn assign_codes(node: &Box<Node>, codes_table: &mut CodeTable, mut code: U8BitVec) {
    if let Some(val) = &node.val {
        codes_table.insert(val.clone(), code);
    } else {
        if let Some(ref left) = node.left {
            code.extend_from_raw_slice(&[0]);
            assign_codes(left, codes_table, code.clone());
        }
        if let Some(ref right) = node.right {
            code.extend_from_raw_slice(&[1]);
            assign_codes(right, codes_table, code.clone());
        }
    }
}
fn nodes_into_tree(mut nodes: Vec<Box<Node>>) -> Box<Node> {
    while nodes.len() > 1 {
        nodes.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        let n1 = nodes.pop().unwrap();
        let n2 = nodes.pop().unwrap();
        let mut parent = Box::new(Node::new(n1.freq + n2.freq, None));
        parent.left = Some(n1);
        parent.right = Some(n2);
        nodes.push(parent);
    }
    return nodes.pop().unwrap();
}


pub fn gen_out_data(file_contents: &String, codes_table: &CodeTable) ->  U8BitVec {
    let mut out: U8BitVec = bitvec![LocalBits, u8;];

    let content_chars: Vec<char> =  file_contents.chars().collect(); 

    for character in content_chars {
        let character = String::from(character);

        add_code(character, &mut out, codes_table);
    }

    add_code(String::from("eof"), &mut out, codes_table);
    return out;
}

fn add_code(char_to_find: String, out_bits: &mut U8BitVec, codes_table: &CodeTable) {
    
    for (ch, code) in codes_table.iter() {
        if char_to_find.eq(ch) {
            out_bits.extend_from_bitslice(code);
        }
    }
}
pub fn to_u8_vec(vectored_data:  U8BitVec) -> Vec<u8> {
    let data_vec = vectored_data.into_vec();
    let mut out_data: Vec<u8> = vec![];
    let base: u8 = 2;
    let mut i = 0;

    // println!("In : {}", data_vec.len());

    while i < data_vec.len() {
        let mut number: u8 = 0;
        for bit_index in 0..8 {
            if i + bit_index < data_vec.len() {
                number += base.pow(bit_index as u32) * data_vec[i + bit_index] ;
            }
        }

        out_data.push(number);
        i += 8;
    }

    // println!("Out: {}", out_data.len());


    return out_data;
}