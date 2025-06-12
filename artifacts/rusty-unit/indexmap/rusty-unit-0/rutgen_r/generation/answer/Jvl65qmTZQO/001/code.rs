// Answer 0

#[test]
fn test_swap_remove_entry_existing_key() {
    use indexmap::IndexMap;

    // Initialize an IndexMap with some key-value pairs
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    // Test for an existing key
    let result = map.swap_remove_entry(&"key1");
    assert_eq!(result, Some(("key1", "value1")));
    assert!(map.get("key1").is_none()); // Ensure the key has been removed
}

#[test]
fn test_swap_remove_entry_second_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    // Test for the second existing key
    let result = map.swap_remove_entry(&"key2");
    assert_eq!(result, Some(("key2", "value2")));
    assert!(map.get("key2").is_none()); // Ensure the key has been removed
}

#[test]
fn test_swap_remove_entry_non_existent_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    
    // Test for a non-existent key
    let result = map.swap_remove_entry(&"non_existent_key");
    assert_eq!(result, None); // Should return None since the key is not present
}

#[test]
fn test_swap_remove_entry_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, &str> = IndexMap::new();
    
    // Test swap_remove_entry on an empty map
    let result = map.swap_remove_entry(&"key1");
    assert_eq!(result, None); // Should return None since the map is empty
}

