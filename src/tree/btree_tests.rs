#[cfg(test)]
pub mod btree_tests {
    use crate::tree::BTree;


    #[test]
    fn it_should_works_with_empty_tree() {
        let tree = BTree::<i32, i32>::new();
        assert_eq!(tree.is_empty(), true);
        assert_eq!(tree.search(4), None);
    }

    #[test]
    fn it_should_works_with_single_element() {
        let mut tree = BTree::new();
        tree.insert(5, "five");
        assert_eq!(tree.search(5), Some(&"five"));
    }

    #[test]
    fn it_should_works_with_multiple_inputs() {
        let mut tree = BTree::new();
        tree.insert(5, "five");
        tree.insert(3, "three");
        tree.insert(7, "seven");

        assert_eq!(tree.search(5), Some(&"five"));
        assert_eq!(tree.search(3), Some(&"three"));
        assert_eq!(tree.search(7), Some(&"seven"));
        assert_eq!(tree.search(4), None);
    }

    #[test]
    fn it_should_works_with_node_splitting() {
        let mut tree = BTree::new();

        for i in 0..10 {
            tree.insert(i, i.to_string());
        }
        
        for i in 0..10 {
            assert_eq!(tree.search(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn it_should_works_with_duplicate_inputs() {
        let mut tree = BTree::new();
        tree.insert(5, "five");
        tree.insert(5, "five");
        assert_eq!(tree.search(5), Some(&"five"));
    }

    #[test]
    fn it_should_works_with_reverse_order() {
        let mut tree = BTree::new();

        for i in (0..10).rev() {
            tree.insert(i, i.to_string());
        }

        for i in (0..10).rev() {
            assert_eq!(tree.search(i), Some(&i.to_string()));
        }
    }
}
