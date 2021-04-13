use std::collections::HashMap;
use bitvec::prelude::*;
use crate::files::control_codes::EOT;


#[derive(Clone, Debug)]
pub struct Node {
    pub val: Option<char>,
    pub freq: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Node {
    pub fn new(freq: usize, ch: Option<char>) -> Node {
        Node {
            freq,
            val: ch,
            left: None,
            right: None,
        }
    }
}

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

    *count_table.entry(EOT as char).or_insert(0) += 1;

    return count_table;
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
