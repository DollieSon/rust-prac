pub struct Node {
    pub elem: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(num: i32) -> Node {
        Node {
            elem: num,
            left: None,
            right: None,
        }
    }
    pub fn new_box(num: i32) -> Box<Node> {
        Box::new(Node::new(num))
    }
}
pub mod BinTree;
