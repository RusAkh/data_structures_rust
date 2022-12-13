mod tests {
    use avl::{self, node::Node, tree};
    #[test]
    fn test_heights() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let root: *mut Node = tree::build_full_tree(values).unwrap();
        println!("ROOT {:?}", unsafe { (*root).value } ) //unsafe{ (*(*root).left.unwrap()).value })
        // assert_ne!(root, None)
    }
}