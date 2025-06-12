// Answer 0

#[test]
fn test_remove_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    
    // Test removing an existing key
    let result = map.remove(&1);
    assert_eq!(result, Some("a"));
    assert_eq!(map.len(), 1); // Verify the length is decremented

    // Test removing a second existing key
    let result = map.remove(&2);
    assert_eq!(result, Some("b"));
    assert!(map.is_empty()); // Verify the map is empty
}

#[test]
fn test_remove_non_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "a");

    // Test removing a non-existing key
    let result = map.remove(&2);
    assert_eq!(result, None); // Verify it returns None
    assert_eq!(map.len(), 1); // Verify the length remains the same
}

