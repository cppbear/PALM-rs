// Answer 0

#[test]
fn test_swap_remove_full_success() {
    struct TestEquivalent;

    impl Equivalent<usize> for TestEquivalent {
        fn equivalent(&self, _other: &usize) -> bool {
            true // Always return true for simplicity
        }
    }

    let mut index_map = IndexMapCore::new();
    let hash = HashValue(1);
    let key = 42;
    let value = "value";

    // Simulate inserting an entry
    index_map.push_entry(hash, key, value);

    // Perform a swap_remove_full operation
    let result = index_map.swap_remove_full(hash, &TestEquivalent);

    assert!(result.is_some());
    let (index, removed_key, removed_value) = result.unwrap();
    assert_eq!(index, 0); // Assuming it's the first entry
    assert_eq!(removed_key, key);
    assert_eq!(removed_value, value);
}

#[test]
fn test_swap_remove_full_not_found() {
    struct TestEquivalent;

    impl Equivalent<usize> for TestEquivalent {
        fn equivalent(&self, _other: &usize) -> bool {
            false // Always return false to ensure not found
        }
    }

    let mut index_map = IndexMapCore::new();
    let hash = HashValue(1);
    let key = 42;

    // Trying to remove an entry that doesn't exist
    let result = index_map.swap_remove_full(hash, &TestEquivalent);

    assert!(result.is_none()); // Should return None since the entry is not found
}

