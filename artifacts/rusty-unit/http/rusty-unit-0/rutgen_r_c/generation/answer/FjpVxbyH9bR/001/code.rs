// Answer 0

#[test]
fn test_remove_entry() {
    #[derive(Debug, Clone)]
    struct TestValue(String);

    let header_name = HeaderName { inner: Repr::Custom }; 
    let mut map = HeaderMap::<TestValue>::new();
    
    // Inserting values
    map.insert(header_name.clone(), TestValue("world".to_string()));
    map.insert(header_name.clone(), TestValue("rust".to_string()));

    // Using the Entry API
    if let OccupiedEntry { map: _, probe: _, index: _ } = map.entry("host").unwrap() {
        let prev_value = occupied_entry.remove(); // remove the first value
        assert_eq!(prev_value, TestValue("world".to_string())); // check if the returned value is as expected
    }

    // Ensuring the key does not exist anymore
    assert!(!map.contains_key("host"));
}

#[test]
#[should_panic]
fn test_remove_entry_nonexistent_key() {
    let header_name = HeaderName { inner: Repr::Custom }; 
    let mut map = HeaderMap::<TestValue>::new();

    // Attempting to remove an entry that doesn't exist
    let entry = map.entry("nonexistent_key"); // This should cause a panic
    entry.remove();
}

