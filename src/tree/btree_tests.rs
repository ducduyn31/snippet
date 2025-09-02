#[cfg(test)]
pub mod tests {
    use crate::tree::BTree;

    #[test]
    fn empty_tree_returns_none_for_searches() {
        let tree = BTree::<i32, i32>::new();
        assert!(tree.is_empty());
        assert_eq!(tree.search(4), None);
    }

    #[test]
    fn single_element_insertion_and_search() {
        let mut tree = BTree::new();
        tree.insert(5, "five");
        assert_eq!(tree.search(5), Some(&"five"));
    }

    #[test]
    fn multiple_elements_basic_operations() {
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
    fn sequential_insertion_with_node_splitting() {
        let mut tree = BTree::new();

        for i in 0..10 {
            tree.insert(i, i.to_string());
        }

        for i in 0..10 {
            assert_eq!(tree.search(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn duplicate_key_insertion_behavior() {
        let mut tree = BTree::new();
        tree.insert(5, "five");
        tree.insert(5, "five");
        assert_eq!(tree.search(5), Some(&"five"));
    }

    #[test]
    fn reverse_order_insertion_maintains_structure() {
        let mut tree = BTree::new();

        for i in (0..10).rev() {
            tree.insert(i, i.to_string());
        }

        for i in (0..10).rev() {
            assert_eq!(tree.search(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn root_leaf_splitting_at_max_capacity() {
        let mut tree = BTree::new();

        tree.insert(1, "one");
        tree.insert(3, "three");
        tree.insert(5, "five");
        tree.insert(2, "two");

        assert_eq!(tree.search(1), Some(&"one"));
        assert_eq!(tree.search(2), Some(&"two"));
        assert_eq!(tree.search(3), Some(&"three"));
        assert_eq!(tree.search(5), Some(&"five"));
    }

    #[test]
    fn multiple_level_tree_construction() {
        let mut tree = BTree::new();

        for i in 1..=20 {
            tree.insert(i, format!("value_{}", i));
        }

        for i in 1..=20 {
            assert_eq!(tree.search(i), Some(&format!("value_{}", i)));
        }
    }

    #[test]
    fn max_degree_boundary_splitting() {
        let mut tree = BTree::new();

        tree.insert(10, "ten");
        tree.insert(20, "twenty");
        tree.insert(30, "thirty");
        tree.insert(15, "fifteen");

        assert_eq!(tree.search(10), Some(&"ten"));
        assert_eq!(tree.search(15), Some(&"fifteen"));
        assert_eq!(tree.search(20), Some(&"twenty"));
        assert_eq!(tree.search(30), Some(&"thirty"));
    }

    #[test]
    fn random_order_insertion_maintains_searchability() {
        let mut tree = BTree::new();

        let keys = [50, 25, 75, 10, 30, 60, 80, 5, 15, 35, 65, 85];

        for key in keys.iter() {
            tree.insert(*key, format!("value_{}", key));
        }

        for key in keys.iter() {
            assert_eq!(tree.search(*key), Some(&format!("value_{}", key)));
        }

        assert_eq!(tree.search(1), None);
        assert_eq!(tree.search(100), None);
    }

    #[test]
    fn duplicate_keys_preserve_first_value() {
        let mut tree = BTree::new();

        tree.insert(5, "first");
        tree.insert(5, "second");
        tree.insert(5, "third");

        assert_eq!(tree.search(5), Some(&"first"));
    }

    #[test]
    fn deep_tree_search_at_all_levels() {
        let mut tree = BTree::new();

        for i in 1..=100 {
            tree.insert(i, format!("deep_{}", i));
        }

        assert_eq!(tree.search(1), Some(&"deep_1".to_string()));
        assert_eq!(tree.search(50), Some(&"deep_50".to_string()));
        assert_eq!(tree.search(100), Some(&"deep_100".to_string()));
        assert_eq!(tree.search(101), None);
    }

    #[test]
    fn empty_tree_comprehensive_search_tests() {
        let tree = BTree::<i32, String>::new();

        assert!(tree.is_empty());
        assert_eq!(tree.search(0), None);
        assert_eq!(tree.search(-1), None);
        assert_eq!(tree.search(1000), None);
    }

    #[test]
    fn sequential_vs_random_insertion_equivalence() {
        let mut sequential_tree = BTree::new();
        let mut random_tree = BTree::new();

        for i in 1..=20 {
            sequential_tree.insert(i, format!("seq_{}", i));
        }

        let random_order = vec![
            11, 3, 17, 8, 14, 1, 19, 6, 12, 20, 4, 15, 9, 2, 18, 7, 13, 10, 5, 16,
        ];
        for i in random_order {
            random_tree.insert(i, format!("seq_{}", i));
        }

        for i in 1..=20 {
            assert_eq!(sequential_tree.search(i), Some(&format!("seq_{}", i)));
            assert_eq!(random_tree.search(i), Some(&format!("seq_{}", i)));
        }
    }

    #[test]
    fn large_dataset_stress_testing() {
        let mut tree = BTree::new();

        for i in 0..1000 {
            tree.insert(i, format!("stress_{}", i));
        }

        let test_keys = vec![0, 1, 99, 100, 250, 500, 750, 999];
        for key in test_keys {
            assert_eq!(tree.search(key), Some(&format!("stress_{}", key)));
        }

        assert_eq!(tree.search(-1), None);
        assert_eq!(tree.search(1000), None);
    }

    #[test]
    fn alternating_pattern_insertion_stability() {
        let mut tree = BTree::new();

        for i in 0..10 {
            tree.insert(i * 2, format!("even_{}", i * 2));
            tree.insert(100 - i, format!("desc_{}", 100 - i));
        }

        for i in 0..10 {
            assert_eq!(tree.search(i * 2), Some(&format!("even_{}", i * 2)));
            assert_eq!(tree.search(100 - i), Some(&format!("desc_{}", 100 - i)));
        }
    }
}
