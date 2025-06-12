// Answer 0

#[test]
fn test_remove_entry() {
    use hashbrown::HashMap;
    use hashbrown::raw::Global;

    // Create a HashMap with string keys and u32 values
    let mut map: HashMap<&str, u32, DefaultHashBuilder, Global> = HashMap::new();

    // Ensure the map is initially empty
    assert!(map.is_empty());

    // Insert a key-value pair into the map
    map.insert("poneyland", 12);

    // Check if the entry exists
    if let Some(mut occupied_entry) = map.get_mut("poneyland") {
        // Remove the entry and assert the returned values
        assert_eq!(occupied_entry.remove_entry(), ("poneyland", 12));
    }

    // Verify the entry has been removed
    assert!(!map.contains_key("poneyland"));
    assert!(map.is_empty());
}

#[test]
fn test_remove_multiple_entries() {
    use hashbrown::HashMap;
    use hashbrown::raw::Global;

    let mut map: HashMap<&str, u32, DefaultHashBuilder, Global> = HashMap::new();

    // Insert multiple key-value pairs
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    // Remove "two" and verify
    if let Some(mut occupied_entry) = map.get_mut("two") {
        assert_eq!(occupied_entry.remove_entry(), ("two", 2));
    }

    // Ensure "two" has been removed
    assert!(!map.contains_key("two"));
    assert_eq!(map.len(), 2); // Should have two entries left
}

#[test]
#[should_panic]
fn test_remove_from_empty_map() {
    use hashbrown::HashMap;
    use hashbrown::raw::Global;

    let mut map: HashMap<&str, u32, DefaultHashBuilder, Global> = HashMap::new();

    // Attempt to remove an entry from an empty map
    // This should panic since there is no occupied entry for "not_found"
    if let Some(mut occupied_entry) = map.get_mut("not_found") {
        occupied_entry.remove_entry(); // This should panic
    }
}

