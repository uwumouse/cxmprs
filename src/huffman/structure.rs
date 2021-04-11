
#[derive(Clone, Debug)]
pub struct Node {
    pub val: Option<String>,
    pub freq: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Node {
    pub fn new(freq: usize, ch: Option<String>) -> Node {
        Node {
            freq,
            val: ch,
            left: None,
            right: None,
        }
    }
}
