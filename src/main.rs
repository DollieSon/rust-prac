mod trees;
use trees::bin_tree::BinTree;
fn main() {
    println!("Hello, world!");
    let mut Tree = BinTree::new();
    Tree.insert_elem(7);
    Tree.insert_elem(3);
    Tree.insert_elem(5);
    Tree.insert_elem(20);
    Tree.insert_elem(31);
    Tree.print_tree();
    Tree.remove_node(5);

    Tree.print_tree();
}
