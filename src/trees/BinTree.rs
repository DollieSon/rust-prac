use crate::trees::Node;
pub struct BinTree {
    head: Option<Box<Node>>,
    node_count: i32,
}

impl BinTree {
    pub fn new() -> BinTree {
        BinTree {
            head: None,
            node_count: 0,
        }
    }
    pub fn insert_node(&mut self, node: Box<Node>) {
        let mut curr_node = &mut self.head;
        while let Some(ref mut unwrapped) = curr_node {
            if node.elem < unwrapped.elem {
                println!("going left of {}", unwrapped.elem);
                curr_node = &mut unwrapped.left;
            } else {
                println!("going right of {}", unwrapped.elem);
                curr_node = &mut unwrapped.right;
            }
        }
        println!("inserting {}", node.elem);
        *curr_node = Some(node);
        self.node_count += 1;
    }
    pub fn insert_elem(&mut self, elem: i32) {
        self.insert_node(Node::new_box(elem));
    }

    pub fn print_tree(&self) {
        let mut pre_container: Vec<i32> = Vec::new();
        let mut in_container: Vec<i32> = Vec::new();
        let mut post_container: Vec<i32> = Vec::new();
        BinTree::print_pre(&self.head, &mut pre_container);
        println!("Pre : {:?}", pre_container);
        BinTree::print_in(&self.head, &mut in_container);
        println!("in : {:?}", in_container);
        BinTree::print_post(&self.head, &mut post_container);
        println!("post : {:?}", post_container);
    }
    fn print_pre(node: &Option<Box<Node>>, container: &mut Vec<i32>) {
        match node {
            Some(n) => {
                container.push(n.elem);
                BinTree::print_pre(&n.left, container);
                BinTree::print_pre(&n.right, container);
            }
            _ => (),
        }
    }
    fn print_in(node: &Option<Box<Node>>, container: &mut Vec<i32>) {
        match node {
            Some(n) => {
                BinTree::print_in(&n.left, container);
                container.push(n.elem);
                BinTree::print_in(&n.right, container);
            }
            _ => (),
        }
    }
    fn print_post(node: &Option<Box<Node>>, container: &mut Vec<i32>) {
        match node {
            Some(n) => {
                BinTree::print_post(&n.left, container);
                BinTree::print_post(&n.right, container);
                container.push(n.elem);
            }
            _ => (),
        }
    }
}
