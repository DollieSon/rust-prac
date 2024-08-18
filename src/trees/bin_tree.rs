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
    // should return sucess?
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
    fn find_mid_node(head: &Box<Node>, dir: char) -> &Option<Box<Node>> {
        match dir.to_ascii_lowercase() {
            'l' => {
                let mut curr_node = &head.left;
                while let Some(ref unwrapped) = curr_node {
                    match unwrapped.left {
                        Some(_) => {
                            curr_node = &unwrapped.left;
                        }
                        _ => {
                            println!("returning Node: {}", unwrapped.elem);
                            return &curr_node;
                        }
                    }
                }
                &curr_node
            }
            'r' => {
                let mut curr_node = &head.right;
                while let Some(ref unwrapped) = curr_node {
                    match unwrapped.right {
                        Some(_) => {
                            curr_node = &unwrapped.right;
                        }
                        _ => {
                            println!("returning Node: {}", unwrapped.elem);
                            return &curr_node;
                        }
                    }
                }
                &curr_node
            }
            _ => {
                panic!("Error Direction");
            }
        }
    }
    pub fn remove_node(&mut self, elem: i32) {
        let mut curr_node = &mut self.head;

        while let Some(ref mut curr) = curr_node {
            if curr.elem > elem {
                // Use a block to limit the scope of the mutable borrow
                curr_node = &mut curr.left;
            } else if curr.elem < elem {
                curr_node = &mut curr.right;
            } else {
                println!("element found {}", curr.elem);
                let mut found: &Option<Box<Node>>;
                match *(curr.as_mut()) {
                    Node {
                        elem: _,
                        left: Some(ref head),
                        right: _,
                    } => {
                        //L inf-R
                        found = BinTree::find_mid_node(head, 'l');
                    }
                    Node {
                        elem: _,
                        left: None,
                        right: Some(ref head),
                    } => {
                        // R inf-L
                        found = BinTree::find_mid_node(head, 'r');
                    }
                    _ => {
                        // just remove and return
                        *curr_node = None;
                        return;
                    }
                }
                match found {
                    Some(node) => {
                        println!("Node : {}", node.elem);
                    }
                    _ => {}
                }
                return;
            }
        }
        println!("element was not found");
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
