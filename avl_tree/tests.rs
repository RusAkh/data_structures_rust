mod tests {
    use avl::{self, node::Node, tree};
    #[test]
    fn test_heights() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let root: Option<*mut Node> = tree::build_full_tree(values);
        assert_ne!(root, None)
    }
}