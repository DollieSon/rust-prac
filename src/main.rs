mod trees;
use trees::BinTree::BinTree;
fn main() {
    println!("Hello, world!");
    let mut tree = BinTree::new();
    tree.insert_elem(32);
    tree.insert_elem(1);
    tree.insert_elem(-1);
    tree.insert_elem(44);
    tree.print_tree();
}
