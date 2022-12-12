use core::num;
use std::cmp;
use crate::node::Node;

pub fn setHeights(root: Option<*mut Node>) -> i64 {
    match root {
        None => -1,
        Some(node) => {
            if num_children(node) == 0 {
                return 0
            } else {
                unsafe {
                    (*node).height = cmp::max(setHeights((*node).left ) , setHeights((*node).right));
                }
                return unsafe { (*node).height }
            }
            
        }
    }
}

fn num_children(root: *mut Node) -> i64 {
    if has_left(root) && has_right(root) {
        2
    } else if has_left(root) || has_right(root) {
        1
    } else {
        0
    }
}

fn has_left(root: *mut Node) -> bool {
    match unsafe{ (*root).left } {
        Some(_node) => true,
        None => false
    }
}

fn has_right(root: *mut Node) -> bool {
    match unsafe{ (*root).right } {
        Some(_node) => true,
        None => false
    }
}

// fn nodeHeight(root: Option<*mut Node>) -> Option<i64> {
//     unimplemented!()
// }
