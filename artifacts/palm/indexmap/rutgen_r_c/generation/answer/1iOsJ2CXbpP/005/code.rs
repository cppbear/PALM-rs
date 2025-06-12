// Answer 0

#[test]
fn test_swap_remove_full() {
    struct DummyKey(u32);
    struct DummyValue(String);

    let mut index_map: IndexMap<DummyKey, DummyValue, RandomState> = IndexMap::new();
    
    // Populate the index map with multiple entries
    index_map.insert(DummyKey(1), DummyValue("One".to_string()));
    index_map.insert(DummyKey(2), DummyValue("Two".to_string()));
    index_map.insert(DummyKey(3), DummyValue("Three".to_string()));

    // Test case for removing an entry, expecting the last entry to be swapped and removed
    let result = index_map.swap_remove_full(&DummyKey(2));
    assert_eq!(result, Some((1, DummyKey(2), DummyValue("Two".to_string())))); // 1 is the index for the removed key

    // Verify that the remaining keys are correct
    assert!(index_map.contains_key(&DummyKey(1)));
    assert!(!index_map.contains_key(&DummyKey(2)));
    assert!(index_map.contains_key(&DummyKey(3)));
    
    // Test case for removing a key that doesn't exist
    let result_none = index_map.swap_remove_full(&DummyKey(4));
    assert_eq!(result_none, None); // Should return None for non-existent key

    // Test case for removing the last key
    let last_result = index_map.swap_remove_full(&DummyKey(3));
    assert_eq!(last_result, Some((0, DummyKey(3), DummyValue("Three".to_string())))); // Should now be the first (and only) index

    // Verify map should now only have one entry
    assert_eq!(index_map.len(), 1);
    
    // Test case for empty map, expecting None
    index_map.swap_remove_full(&DummyKey(1)); // Remove the last remaining entry
    let result_empty = index_map.swap_remove_full(&DummyKey(1));
    assert_eq!(result_empty, None); // Should return None since map is now empty
}

