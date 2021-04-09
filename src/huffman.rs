use std::collections::HashMap;
use bitvec::prelude::*;

#[derive(Clone, Debug)]
pub struct Node {
    pub val: Option<String>,
    pub freq: isize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn has_children(&self) -> bool {
        return self.right.is_some() && self.left.is_none();
    }

    fn get_left(&self) -> Box<Node> {
        self.left.clone().unwrap()
    }

    fn get_right(&self) -> Box<Node> {
        self.right.clone().unwrap()
    }
}

#[derive(Debug)]
pub struct Code(String, BitVec);

pub fn generate_tree(content: &String) -> Node {
    let mut letter_counts: HashMap<char,isize> = HashMap::new();

    // List of every char in string
    let chars: Vec<char> = content.chars().collect();

    // Counting occurences of every character in string
    for ch in chars {
        *letter_counts.entry(ch).or_insert(0) += 1;
    }
    
    // Creating vector from letters and their counts
    let mut nodes: Vec<Node> = letter_counts
        .into_iter()
        .map(|(ch, freq)| {
            return Node {
                val: Some(ch.to_string()),
                freq: freq,
                right: None,
                left: None
            }
        })
        .collect();

    nodes.push(Node {
        val: Some(String::from("eof")),
        freq: 1,
        right: None,
        left: None,
    });

    nodes = sort_nodes(nodes);

    return nodes_into_tree(nodes);
}

pub fn tree_to_codes(tree: &mut Node) -> Vec<Code> {
    let mut codes: Vec<Code> = vec![];


    for _ in 0..tree.freq {
        let code = find_leaf(tree);
        println!("{:#?}", code);
        codes.push(code);
    }
    
    println!("Tree after encoding: {:#?}", tree);

    // for code in codes.iter() {
    // }
    return codes;
}

fn find_leaf(tree: &mut Node) -> Code {
    let mut code = bitvec![];
    let mut curr_node = tree;

    loop {
        if !curr_node.has_children() {
            let result = Code(curr_node.val.clone().unwrap(), code.clone()); 
            
            drop(curr_node);

            return result;
        }

        let mut code_part = 1;
        let mut is_right = true;
        let mut child: Box<Node>;


        if curr_node.right.is_some() {
            child = curr_node.get_right();
        } else if curr_node.left.is_some() {
            is_right = false;
            code_part = 0;
            child = curr_node.get_left();
        } else {
            panic!();
        }

        code.extend_from_raw_slice(&[code_part]);

        if !child.has_children() {
            if is_right {
                curr_node.right = None;
            } else {
                curr_node.left = None;
            }

            // TODO: Fix "does not live long enough" error
            curr_node = child.as_mut();
        }

    }
}

fn sort_nodes(mut nodes: Vec<Node>) -> Vec<Node> {
    nodes.sort_by(|a, b| a.freq.cmp(&b.freq));
    return nodes;
}


fn nodes_into_tree(mut nodes: Vec<Node>) -> Node {
    if nodes.len() == 1 {
        return nodes[0].clone();
    }
    
    let left = Some(Box::new(nodes[0].clone()));
    let right = Some(Box::new(nodes[1].clone()));

    let parent_node = Node {
        freq: nodes[0].freq + nodes[1].freq,
        val: None,
        left,
        right
    };
    // Remove two least freq chars
    nodes.remove(0);
    nodes.remove(0);

    // Add new instead
    nodes.push(parent_node);
    // Sort again
    nodes = sort_nodes(nodes);

    return nodes_into_tree(nodes);
}