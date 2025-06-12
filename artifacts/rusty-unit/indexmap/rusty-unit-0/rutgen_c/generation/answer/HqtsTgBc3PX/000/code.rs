// Answer 0

#[test]
fn test_shift_remove_full_existing_entry() {
    let mut map: IndexMapCore<i32, &str> = IndexMapCore::new();
    let hash = HashValue(1);
    
    // Insert an entry
    map.insert_full(hash, 1, "value1");
    
    // Try to shift remove the existing entry
    let result = map.shift_remove_full(hash, &1);
    
    // Check the result
    assert_eq!(result, Some((0, 1, "value1")));
    assert_eq!(map.len(), 0); // Map should be empty after removal
}

#[test]
fn test_shift_remove_full_non_existing_entry() {
    let mut map: IndexMapCore<i32, &str> = IndexMapCore::new();
    let hash = HashValue(1);
    
    // Insert an entry
    map.insert_full(hash, 1, "value1");
    
    // Try to shift remove a non-existing entry
    let result = map.shift_remove_full(hash, &2);
    
    // Check the result
    assert_eq!(result, None);
    assert_eq!(map.len(), 1); // Map should still contain the first entry
}

#[test]
fn test_shift_remove_full_multiple_entries() {
    let mut map: IndexMapCore<i32, &str> = IndexMapCore::with_capacity(3);
    let hash1 = HashValue(1);
    let hash2 = HashValue(2);
    
    // Insert multiple entries
    map.insert_full(hash1, 1, "value1");
    map.insert_full(hash2, 2, "value2");
    
    // Remove the first entry
    let result = map.shift_remove_full(hash1, &1);
    
    // Check the result
    assert_eq!(result, Some((0, 1, "value1")));
    assert_eq!(map.len(), 1); // Map should have one entry left
    
    // Check the remaining entry
    let remaining_result = map.shift_remove_full(hash2, &2);
    assert_eq!(remaining_result, Some((0, 2, "value2")));
    assert_eq!(map.len(), 0); // Map should be empty after removing all entries
}

