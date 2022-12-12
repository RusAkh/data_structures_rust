mod tests {
    use avl::{self, node::Node, tree};
    #[test]
    fn TestHeights() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let root: *mut Node = tree::build_tree_with_balances(values);
    }
}