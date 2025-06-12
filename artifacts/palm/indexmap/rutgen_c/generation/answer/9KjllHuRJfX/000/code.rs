// Answer 0

#[test]
fn test_swap_remove_full_entry_exists() {
    struct TestEquivalent;
    impl Equivalent<usize> for TestEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            true
        }
    }

    let mut index_map = IndexMapCore::new();
    index_map.insert_full(HashValue(1), 1, 10);
    index_map.insert_full(HashValue(2), 2, 20);

    let result = index_map.swap_remove_full(HashValue(1), &TestEquivalent);
    assert_eq!(result, Some((0, 1, 10)));
    assert_eq!(index_map.len(), 1);  // Ensure the item was removed.
}

#[test]
fn test_swap_remove_full_entry_not_exists() {
    struct TestEquivalent;
    impl Equivalent<usize> for TestEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            false
        }
    }

    let mut index_map = IndexMapCore::new();
    index_map.insert_full(HashValue(1), 1, 10);
    
    let result = index_map.swap_remove_full(HashValue(2), &TestEquivalent);
    assert_eq!(result, None);  // Ensure no item was removed.
}

#[test]
fn test_swap_remove_full_empty() {
    struct TestEquivalent;
    impl Equivalent<usize> for TestEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            false
        }
    }

    let mut index_map = IndexMapCore::new();
    let result = index_map.swap_remove_full(HashValue(1), &TestEquivalent);
    assert_eq!(result, None);  // Ensure no item is removed from empty map.
}

