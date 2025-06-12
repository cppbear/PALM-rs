// Answer 0

#[test]
fn test_insert_full_vacant_entry() {
    struct TestKey(usize);
    struct TestValue(usize);

    let mut index_map = IndexMapCore::<TestKey, TestValue>::new();
    let hash_value = HashValue(1);
    let key = TestKey(1);
    let value = TestValue(10);

    // Initial insert should yield a vacated entry.
    let (index, existing_value) = index_map.insert_full(hash_value, key, value);

    // Validate the results.
    assert_eq!(index, 0); // This is the first insertion, so index should be 0.
    assert!(existing_value.is_none()); // No existing value should be present.
}

#[test]
fn test_insert_full_overwrite_existing_value() {
    struct TestKey(usize);
    struct TestValue(usize);

    let mut index_map = IndexMapCore::<TestKey, TestValue>::new();
    let hash_value = HashValue(1);
    let key = TestKey(1);
    let value1 = TestValue(10);
    let value2 = TestValue(20);

    // Insert first value
    let (index1, existing_value1) = index_map.insert_full(hash_value, key, value1);
    assert_eq!(index1, 0);
    assert!(existing_value1.is_none());

    // Insert second value, which should overwrite the first.
    let (index2, existing_value2) = index_map.insert_full(hash_value, key, value2);
    assert_eq!(index2, 0); // Should still be index 0
    assert_eq!(existing_value2, Some(value1)); // The existing value should be the first value
}

#[test]
fn test_insert_full_multiple_entries() {
    struct TestKey(usize);
    struct TestValue(usize);

    let mut index_map = IndexMapCore::<TestKey, TestValue>::new();
    let hash_value1 = HashValue(1);
    let key1 = TestKey(1);
    let value1 = TestValue(10);
    
    let hash_value2 = HashValue(2);
    let key2 = TestKey(2);
    let value2 = TestValue(20);

    // Insert first entry
    let (index1, existing_value1) = index_map.insert_full(hash_value1, key1, value1);
    assert_eq!(index1, 0);
    assert!(existing_value1.is_none());

    // Insert second entry
    let (index2, existing_value2) = index_map.insert_full(hash_value2, key2, value2);
    assert_eq!(index2, 1); // Should be at index 1 now
    assert!(existing_value2.is_none()); // No existing value to replace
}

#[test]
fn test_insert_full_panic_condition() {
    struct TestKey(usize);
    struct TestValue(usize);

    let mut index_map = IndexMapCore::<TestKey, TestValue>::new();
    let hash_value = HashValue(1);
    let key1 = TestKey(1);
    let value1 = TestValue(10);

    // Insert first entry to create a vacant entry
    index_map.insert_full(hash_value, key1, value1);

    // Attempt to insert an entry that would cause the panic condition
    // This will require implementing eq for keys in a similar way.
    let key2 = TestKey(1); // Same as the first key, causing an "Occupied" condition.
    
    // This should overwrite the existing entry without panic.
    let (index, existing_value) = index_map.insert_full(hash_value, key2, TestValue(30));
    assert_eq!(index, 0); // Should still be index 0
    assert_eq!(existing_value, Some(value1)); // Old value should be returned
}

