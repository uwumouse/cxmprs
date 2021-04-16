use std::collections::HashMap;
use crate::CharMap;
use crate::control_codes::*;

/**
	Goes through every character in header, collecting ne-bytes for it and turninng them
	into integers.  
	Stops reading when meets [NULL][US] bytes, 
	
	Returns generated character map and index of the last byte in header (next byte will be starting of data) 
*/
pub fn header_to_charmap(compress_bytes: &Vec<u8>) -> (CharMap, usize) {
	// Variable for storing "char: freq" data from header
	let mut char_map: CharMap = HashMap::new();
	// Index of current byte, will be returned as pointer to last byte in header
	let mut byte_index = 0;

	// Repersents current char. By default it's first byte in file.
	let mut curr_char = compress_bytes[0] as char;
	// ne-bytes for character, needed to return actual frequency instead of encoded bytes
	let mut char_ne_bytes: [u8; 8] = Default::default();

	/*  
		current ne-byte index, by default array of bytes will be all zeroes, so wee need to know
		where to put current ne-byte 
	*/
	let mut ne_byte_index = 0;

	/*
		Defines if next bytes in sequence represents char (set to true) or frequency (set to false)
		Default is true since the first character is char anyway
	*/
	let mut is_char = true;
	for i in 0..compress_bytes.len() {
		let byte = compress_bytes[i];
		// Keeping track of current byte for returning at the end
		byte_index = i;

		if is_char {
			// Unit-Separator means that header is ended
			if byte == US {
				break;
			}
			// Saving current char to update charmap in future
			curr_char = byte as char;
			// The next byte will be ne-byte, so setting index for future operations
			ne_byte_index = 0;

			// Creating entry in charmap
			char_map.entry(curr_char).or_insert(0);
			// Next byte isn't char
			is_char = false;
			continue;
		}

		// Group Seperator means ne-bytes are ended for current char
		if byte == GS {
			// Next char will be character
			is_char = true;

			// Setting frequency to integer from ne-bytes
			*char_map.entry(curr_char).or_insert(0) = usize::from_ne_bytes(char_ne_bytes);

			// Resetting ne-bytes for character since we're going to new one
			char_ne_bytes = Default::default();

			continue;
		}

		
		char_ne_bytes[ne_byte_index] = byte;
		ne_byte_index += 1;
	}

	return (char_map, byte_index);
}