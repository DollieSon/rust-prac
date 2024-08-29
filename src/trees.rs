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
    // I dont understand this anymore ..Magic Happens Here
    pub fn right_most(&mut self) -> Option<Box<Node>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(r) = right.right_most() {
                    Some(r)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = std::mem::replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }
}
pub mod bin_tree;
