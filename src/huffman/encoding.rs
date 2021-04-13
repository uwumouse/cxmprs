use crate::huffman::structure::*;
use crate::files::control_codes::EOT;
use bitvec::prelude::*;

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

// Generate final vector of bytes
pub fn gen_out_data(file_contents: &String, codes_table: &CodeTable) -> Vec<u8> {
    let mut out_bits: U8BitVec = bitvec![LocalBits, u8;];

    let content_chars: Vec<char> = file_contents.chars().collect();

    for ch in content_chars {

        add_code(&mut out_bits, codes_table, ch);
    }

    add_code(&mut out_bits, codes_table, EOT as char);

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
