// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    // Define a simple struct for the keys and values
    struct KeyType(i32);
    struct ValueType(&'static str);

    // Create a hash map with integer keys and string values
    let mut map: HashMap<KeyType, ValueType> = HashMap::new();
    map.insert(KeyType(1), ValueType("a"));

    // Remove an entry by its key
    let result = map.remove_entry(&KeyType(1));

    // Check that the returned value is as expected
    assert_eq!(result, Some((KeyType(1), ValueType("a"))));
}

#[test]
fn test_remove_entry_non_existent_key() {
    // Define a simple struct for the keys and values
    struct KeyType(i32);
    struct ValueType(&'static str);

    // Create a hash map with integer keys and string values
    let mut map: HashMap<KeyType, ValueType> = HashMap::new();
    map.insert(KeyType(1), ValueType("a"));

    // Try to remove a non-existent key
    let result = map.remove_entry(&KeyType(2));

    // Check that the returned value is None
    assert_eq!(result, None);
}

#[test]
fn test_remove_entry_key_reuse() {
    // Define a simple struct for the keys and values
    struct KeyType(i32);
    struct ValueType(&'static str);

    // Create a hash map with integer keys and string values
    let mut map: HashMap<KeyType, ValueType> = HashMap::new();
    map.insert(KeyType(1), ValueType("a"));
    map.remove_entry(&KeyType(1)); // Remove the entry

    // Check that the map is empty now
    assert!(map.remove(&KeyType(1)).is_none());
}

