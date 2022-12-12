use crate::height::setHeights;

// use crate::height::height::setHeights;

use crate::node::Node;
fn build_tree(arr: &Vec<i64>, i: usize, n: usize) -> Option<*mut Node> {
    let mut root: *mut Node = &mut Node{
        value: 0,
        right: None,
        left: None,
        height: 0,
        balance: 0,
    };
    if i < n {
        unsafe {
            (*root).value = arr[i];
            (*root).left = build_tree(arr, 2*i + 1, n);
            (*root).right = build_tree(arr, 2*i + 2, n);
        }
    } else {
        ()
    }
    Some(root)
}

pub fn build_full_tree(arr: Vec<i64>) -> Option<*mut Node> {
    let n = arr.len();
    let root = build_tree(&arr, 0, n);
    setHeights(root);
    unimplemented!()
}
    