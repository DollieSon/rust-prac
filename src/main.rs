mod trees;
use trees::{
    bin_tree::BinTree,
    tree::{self, Tree},
};
fn main() {
    test1::<BinTree>();
}
//Test for correct ordering
fn test1<T: Tree>() {
    println!("Test1 : adding elements and getting pre-post-in order");
    let mut a = T::new();
    a.add_elem(1);
    a.add_elem(45);
    a.add_elem(13);
    a.add_elem(-1);
    a.add_elem(44);
    a.add_elem(70);
    let mut pre = Vec::<i32>::new();
    let mut post = Vec::<i32>::new();
    let mut in_con = Vec::<i32>::new();
    // a.print();
    a.get_in(&mut in_con);
    a.get_pre(&mut pre);
    a.get_post(&mut post);

    let target_pre = vec![1, -1, 45, 13, 44, 70];
    let target_in = vec![-1, 1, 13, 44, 45, 70];
    let target_post = vec![-1, 44, 13, 70, 45, 1];
    // println!("pre: {:?}", pre);
    // println!("target_pre: {:?}", target_pre);
    assert!((target_pre.eq(&pre)), "Pre-Order is incorrrect");
    assert!((target_in.eq(&in_con)), "In-Order is incorrrect");
    assert!((target_post.eq(&post)), "Pre-Order is incorrrect");
    println!("Test! : PASSED")
    // Pre : [1, -1, 45, 13, 44, 70]
    // in : [-1, 1, 13, 44, 45, 70]
    // post : [-1, 44, 13, 70, 45, 1]
}

fn test2<T: Tree>() {}
