use crate::trees::tree::Tree;
use crate::trees::Node;
use std::mem;
pub struct BinTree {
    head: Option<Box<Node>>,
    node_count: i32,
}
// I HATE TESTS NO TESTS
// THIS WORKS TRUST GOD
impl BinTree {
    pub fn new() -> BinTree {
        BinTree {
            head: None,
            node_count: 0,
        }
    }
    // should return sucess?
    pub fn insert_node(&mut self, node: Box<Node>) {
        let mut curr_node = &mut self.head;
        while let Some(ref mut unwrapped) = curr_node {
            if node.elem < unwrapped.elem {
                // println!("going left of {}", unwrapped.elem);
                curr_node = &mut unwrapped.left;
            } else {
                // println!("going right of {}", unwrapped.elem);
                curr_node = &mut unwrapped.right;
            }
        }
        // println!("inserting {}", node.elem);
        *curr_node = Some(node);
        self.node_count += 1;
    }
    pub fn insert_elem(&mut self, elem: i32) {
        self.insert_node(Node::new_box(elem));
    }
    pub fn invert_tree(&mut self) {
        self.head = BinTree::invert_node(&mut self.head);
    }
    fn invert_node(node: &mut Option<Box<Node>>) -> Option<Box<Node>> {
        match node {
            Some(unwrapped) => {
                let temp_left = BinTree::invert_node(&mut unwrapped.left);
                unwrapped.left = BinTree::invert_node(&mut unwrapped.right);
                unwrapped.right = temp_left;
                return node.take();
            }
            _ => None,
        }
    }
    //Unfinished - cannot actually remove
    pub fn remove_node(&mut self, elem: i32) {
        if let Some(root) = self.head.take() {
            self.head = BinTree::rem_node(root, elem);
        }
    }
    //recurssive remove
    fn rem_node(mut curr: Box<Node>, elem: i32) -> Option<Box<Node>> {
        if elem < curr.elem {
            if let Some(left) = curr.left.take() {
                curr.left = BinTree::rem_node(left, elem);
            }
            return Some(curr);
        }
        if elem > curr.elem {
            if let Some(right) = curr.right.take() {
                curr.right = BinTree::rem_node(right, elem);
            }
            return Some(curr);
        }
        // I dont understand this part
        // magic happens here
        match (curr.left.take(), curr.right.take()) {
            (None, None) => None,
            (None, Some(right)) => Some(right),
            (Some(left), None) => Some(left),
            (Some(mut left), Some(right)) => {
                if let Some(mut rm) = left.right_most() {
                    rm.left = Some(left);
                    rm.right = Some(right);
                    Some(rm)
                } else {
                    left.right = Some(right);
                    Some(left)
                }
            }
        }
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
    pub fn print_pre(node: &Option<Box<Node>>, container: &mut Vec<i32>) {
        match node {
            Some(n) => {
                container.push(n.elem);
                BinTree::print_pre(&n.left, container);
                BinTree::print_pre(&n.right, container);
            }
            _ => (),
        }
    }
    pub fn print_in(node: &Option<Box<Node>>, container: &mut Vec<i32>) {
        match node {
            Some(n) => {
                BinTree::print_in(&n.left, container);
                container.push(n.elem);
                BinTree::print_in(&n.right, container);
            }
            _ => (),
        }
    }
    pub fn print_post(node: &Option<Box<Node>>, container: &mut Vec<i32>) {
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

// I know this is repetitive
// I KNOW....
impl Tree for BinTree {
    fn new() -> Self {
        BinTree::new()
    }
    fn add_elem(&mut self, elem: i32) {
        self.insert_elem(elem);
    }
    fn remove_elem(&mut self, elem: i32) {
        self.remove_node(elem);
    }
    fn get_depth(&self) -> i32 {
        return 0; // TODO
    }
    fn get_pre(&self, container: &mut Vec<i32>) {
        BinTree::print_pre(&self.head, container);
    }
    fn get_post(&self, container: &mut Vec<i32>) {
        BinTree::print_post(&self.head, container);
    }
    fn get_in(&self, container: &mut Vec<i32>) {
        BinTree::print_in(&self.head, container);
    }
    fn print(&self) {
        self.print_tree();
    }
}
