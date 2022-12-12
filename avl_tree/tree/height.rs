// pub mod height {
//     use crate::node::node::Node;

//     pub fn setHeights(root: Option<*mut Node>) {
//         match root {
//             None => -1,
//             Some(node) => {
//                 if match root.left {
//                     Some(node) => true,
//                     None => false
//                 } && match root.right {
//                     Some(node) => true,
//                     None => false
//                 }{
//                     0
//                 } else {
//                     root.height = cmp::max(nodeHeight(root.left), nodeHeight(root.right))
//                 }
//             }
//         }
//     }
// }

