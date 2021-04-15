use bitvec::prelude::*;
use crate::huffman::structure::*;
use crate::files::control_codes::EOT;

// Generate final vector of bytes
pub fn gen_out_data(file_contents: &String, codes_table: &CodeTable) -> Vec<u8> {
	let mut out_bits: U8BitVec = bitvec![LocalBits, u8;];

	let content_chars: Vec<char> = file_contents.chars().collect();

	for ch in content_chars {
		out_bits.extend_from_bitslice(codes_table.get(&ch).unwrap());
	}

	out_bits.extend_from_bitslice(codes_table.get(&(EOT as char)).unwrap());

	return to_bytes_vec(out_bits);
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
		let mut number: u8 = 0; // Number current byte will represent
		let remainer = if len - i > 8 { 8 } else { len - i }; // How many bits are avaliable to put in byte

		for bit_index in 0..remainer {
			number += base.pow(bit_index as u32) * data_vec[i + bit_index];
		}

		bytes.push(number);

		i += 8;
	}

	return bytes;
}
