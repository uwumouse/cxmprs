
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
