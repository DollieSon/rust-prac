pub trait Tree {
    fn new() -> Self;
    fn add_elem(&mut self, elem: i32);
    fn remove_elem(&mut self, elem: i32);
    fn get_depth(&self) -> i32;
    fn get_pre(&self, container: &mut Vec<i32>);
    fn get_post(&self, container: &mut Vec<i32>);
    fn get_in(&self, container: &mut Vec<i32>);
    fn print(&self);
}
