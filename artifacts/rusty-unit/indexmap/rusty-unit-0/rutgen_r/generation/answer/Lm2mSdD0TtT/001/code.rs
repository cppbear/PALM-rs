// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    // Ensure the entry at index 1 can be removed
    assert_eq!(map.shift_remove_index(1), Some(("key2", "value2")));
    
    // After removal, we expect the map to have 2 entries and key2 to not exist
    assert_eq!(map.len(), 2);
    assert!(!map.contains_key("key2"));
}

#[test]
fn test_shift_remove_index_removes_last_entry() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    // Remove the last entry
    assert_eq!(map.shift_remove_index(1), Some(("key2", "value2")));
    
    // Check resultant length and last key
    assert_eq!(map.len(), 1);
    assert!(map.contains_key("key1"));
}

#[test]
fn test_shift_remove_index_empty_map() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();

    // Attempt to remove from an empty map, expecting None
    assert_eq!(map.shift_remove_index(0), None);
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");

    // Attempting to remove from index 1 in a map with only one element should panic
    map.shift_remove_index(1);
} 

#[test]
fn test_shift_remove_index_boundary_case() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");

    // Remove the only entry in the map
    assert_eq!(map.shift_remove_index(0), Some(("key1", "value1")));

    // After removal, check that the map is empty
    assert_eq!(map.len(), 0);
}

