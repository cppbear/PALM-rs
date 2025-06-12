// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    // Attempt to remove an existing entry
    let result = map.remove_entry(&1);
    assert_eq!(result, Some((1, "a")));
    assert!(map.is_empty()); // Map should be empty after removal
}

#[test]
fn test_remove_entry_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    // Attempt to remove a non-existing key
    let result = map.remove_entry(&2);
    assert_eq!(result, None);
    assert_eq!(map.len(), 1); // Map should still have one entry
}

#[test]
fn test_remove_entry_after_removal() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.remove_entry(&1); // Remove the existing entry

    // Attempt to remove the same key again
    let result = map.remove_entry(&1);
    assert_eq!(result, None); // Should return None again
    assert!(map.is_empty()); // Map should be empty
}

#[test]
fn test_remove_entry_edge_case_empty_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();

    // Attempt to remove from an empty map
    let result = map.remove_entry(&1);
    assert_eq!(result, None); // Should return None
    assert!(map.is_empty()); // Map should remain empty
}

