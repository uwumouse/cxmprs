use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    pub val: String,
    pub freq: isize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub fn get_tree(content: &String) -> Node {
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
                val: ch.to_string(),
                freq: freq,
                right: None,
                left: None
            }
        })
        .collect();

    nodes.push(Node {
        val: String::from("eof"),
        freq: 1,
        right: None,
        left: None,
    });

    nodes = sort_nodes(nodes);

    return create_tree(nodes);
}

fn sort_nodes(mut nodes: Vec<Node>) -> Vec<Node> {
    nodes.sort_by(|a, b| a.freq.cmp(&b.freq));
    return nodes;
}


fn create_tree(mut nodes: Vec<Node>) -> Node {
    if nodes.len() == 1 {
        return nodes[0].clone();
    }
    
    let parent_node = Node {
        freq: nodes[0].freq + nodes[1].freq,
        val: String::new(),
        left: Some(Box::new(nodes[0].clone())),
        right: Some(Box::new(nodes[1].clone()))
    };
    // Remove two least freq chars
    nodes.remove(0);
    nodes.remove(0);

    // Add new instead
    nodes.push(parent_node);
    // Sort again
    nodes = sort_nodes(nodes);

    return create_tree(nodes);
}