#[cfg(test)]
pub mod tests {
    use crate::tree::BTree;


    #[test]
    fn it_should_works_with_empty_tree() {
        let tree = BTree::<i32, i32>::new();
        assert!(tree.is_empty());
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

    #[test]
    fn test_root_leaf_splitting() {
        let mut tree = BTree::new();
        
        // Insert exactly max keys to trigger root split
        tree.insert(1, "one");
        tree.insert(3, "three");
        tree.insert(5, "five");
        
        // This should trigger root split since MAX_DEGREE = 2, max keys = 3
        tree.insert(2, "two");
        
        assert_eq!(tree.search(1), Some(&"one"));
        assert_eq!(tree.search(2), Some(&"two"));
        assert_eq!(tree.search(3), Some(&"three"));
        assert_eq!(tree.search(5), Some(&"five"));
    }

    #[test]
    fn test_multiple_level_splitting() {
        let mut tree = BTree::new();
        
        // Insert enough elements to create multiple levels
        for i in 1..=20 {
            tree.insert(i, format!("value_{}", i));
        }
        
        // Verify all elements are searchable
        for i in 1..=20 {
            assert_eq!(tree.search(i), Some(&format!("value_{}", i)));
        }
    }

    #[test]
    fn test_boundary_conditions() {
        let mut tree = BTree::new();
        
        // Test with exactly MAX_DEGREE - 1 keys (shouldn't split)
        tree.insert(10, "ten");
        
        // Test with MAX_DEGREE keys (should split on next insert)
        tree.insert(20, "twenty");
        tree.insert(30, "thirty");
        
        // This should trigger a split
        tree.insert(15, "fifteen");
        
        assert_eq!(tree.search(10), Some(&"ten"));
        assert_eq!(tree.search(15), Some(&"fifteen"));
        assert_eq!(tree.search(20), Some(&"twenty"));
        assert_eq!(tree.search(30), Some(&"thirty"));
    }

    #[test]
    fn test_non_uniform_key_distribution() {
        let mut tree = BTree::new();
        
        // Insert keys in non-sequential order to test splitting logic
        let keys = vec![50, 25, 75, 10, 30, 60, 80, 5, 15, 35, 65, 85];
        
        for key in keys.iter() {
            tree.insert(*key, format!("value_{}", key));
        }
        
        // Verify all keys are findable
        for key in keys.iter() {
            assert_eq!(tree.search(*key), Some(&format!("value_{}", key)));
        }
        
        // Test some non-existent keys
        assert_eq!(tree.search(1), None);
        assert_eq!(tree.search(100), None);
    }

    #[test]
    fn test_duplicate_key_handling() {
        let mut tree = BTree::new();
        
        tree.insert(5, "first");
        tree.insert(5, "second");
        tree.insert(5, "third");
        
        // Should find the first inserted value (or last, depending on implementation)
        assert_eq!(tree.search(5), Some(&"first"));
    }

    #[test]
    fn test_search_in_deep_tree() {
        let mut tree = BTree::new();
        
        // Create a deeper tree structure
        for i in 1..=100 {
            tree.insert(i, format!("deep_{}", i));
        }
        
        // Test search at various levels
        assert_eq!(tree.search(1), Some(&"deep_1".to_string()));
        assert_eq!(tree.search(50), Some(&"deep_50".to_string()));
        assert_eq!(tree.search(100), Some(&"deep_100".to_string()));
        assert_eq!(tree.search(101), None);
    }

    #[test]
    fn test_empty_tree_operations() {
        let tree = BTree::<i32, String>::new();
        
        assert!(tree.is_empty());
        assert_eq!(tree.search(0), None);
        assert_eq!(tree.search(-1), None);
        assert_eq!(tree.search(1000), None);
    }

    #[test]
    fn test_sequential_vs_random_insertion() {
        let mut sequential_tree = BTree::new();
        let mut random_tree = BTree::new();
        
        // Sequential insertion
        for i in 1..=20 {
            sequential_tree.insert(i, format!("seq_{}", i));
        }
        
        // Random insertion (same keys, different order)
        let random_order = vec![11, 3, 17, 8, 14, 1, 19, 6, 12, 20, 
                               4, 15, 9, 2, 18, 7, 13, 10, 5, 16];
        for i in random_order {
            random_tree.insert(i, format!("seq_{}", i));
        }
        
        // Both trees should find all elements
        for i in 1..=20 {
            assert_eq!(sequential_tree.search(i), Some(&format!("seq_{}", i)));
            assert_eq!(random_tree.search(i), Some(&format!("seq_{}", i)));
        }
    }

    #[test]
    fn test_large_dataset_stress() {
        let mut tree = BTree::new();
        
        // Insert a larger dataset to stress test
        for i in 0..1000 {
            tree.insert(i, format!("stress_{}", i));
        }
        
        // Verify random sampling of elements
        let test_keys = vec![0, 1, 99, 100, 250, 500, 750, 999];
        for key in test_keys {
            assert_eq!(tree.search(key), Some(&format!("stress_{}", key)));
        }
        
        // Test non-existent keys
        assert_eq!(tree.search(-1), None);
        assert_eq!(tree.search(1000), None);
    }

    #[test]
    fn test_alternating_pattern_insertion() {
        let mut tree = BTree::new();
        
        // Insert in alternating high-low pattern
        for i in 0..10 {
            tree.insert(i * 2, format!("even_{}", i * 2));
            tree.insert(100 - i, format!("desc_{}", 100 - i));
        }
        
        // Verify all insertions
        for i in 0..10 {
            assert_eq!(tree.search(i * 2), Some(&format!("even_{}", i * 2)));
            assert_eq!(tree.search(100 - i), Some(&format!("desc_{}", 100 - i)));
        }
    }
}
