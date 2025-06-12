// Answer 0

#[test]
fn test_replace_full_vacant_entry() {
    #[derive(Debug, Clone, PartialEq)]
    struct TestKey(usize);
    
    #[derive(Debug, Clone, PartialEq)]
    struct TestValue(usize);

    // Create a new IndexMapCore instance
    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();

    // Create a hash value
    let hash_value = HashValue(1);

    // Attempt to replace a value for a key not yet present
    let result = index_map.replace_full(hash_value, TestKey(42), TestValue(100));

    // Verify the result
    assert_eq!(result, (0, None));
    assert_eq!(index_map.len(), 1); // Ensure one entry is added
}

#[test]
fn test_replace_full_occupied_entry() {
    #[derive(Debug, Clone, PartialEq)]
    struct TestKey(usize);
    
    #[derive(Debug, Clone, PartialEq)]
    struct TestValue(usize);

    // Create a new IndexMapCore instance and insert an initial entry
    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash_value = HashValue(1);
    index_map.push_entry(hash_value, TestKey(42), TestValue(100));

    // Replacing the existing key
    let result = index_map.replace_full(hash_value, TestKey(42), TestValue(200));

    // Verify the result
    assert_eq!(result, (0, Some((TestKey(42), TestValue(100))))); // Verify replaced value
    assert_eq!(index_map.len(), 1); // Ensure entry count remains the same
}

