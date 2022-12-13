use std::cmp;
use crate::node::Node;

pub fn set_heights(root: Option<*mut Node>) -> i64 {
    match root {
        None => -1,
        Some(node) => {
            if num_children(unsafe { &mut *node }) == 0 {
                0
            } else {
                unsafe {
                    (*node).height = cmp::max(set_heights((*node).left ) , set_heights((*node).right));
                };
                unsafe { (*node).height }
            }
            
        }
    }
}

fn num_children(root: &mut Node) -> i64 {
    if has_left(root) && has_right(root) {
        2
    } else if has_left(root) || has_right(root) {
        1
    } else {
        0
    }
}

fn has_left(root: &mut Node) -> bool {
    println!("{:?}", root);
    match root.left {
        Some(_) => true,
        None => false
    }
}

fn has_right(root: &mut Node) -> bool {
    match root.right {
        Some(_) => true,
        None => false
    }
}
