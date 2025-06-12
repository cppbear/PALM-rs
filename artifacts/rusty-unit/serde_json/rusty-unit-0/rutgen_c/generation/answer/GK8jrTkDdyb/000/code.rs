// Answer 0

#[test]
fn test_index_into_mut_string() {
    struct TestIndexing;

    impl private::Sealed for TestIndexing {}
    
    let key = String::from("key");
    let mut value = Value::Object(Map::new());
    
    // Test case where the key does not exist in the object
    let result = key.index_into_mut(&mut value);
    assert!(result.is_none());

    // Insert a key-value pair into the object for further testing
    if let Value::Object(ref mut map) = value {
        map.insert(key.clone(), Value::Bool(true));
    }

    // Test case where the key exists in the object
    let result = key.index_into_mut(&mut value);
    assert!(result.is_some());
    
    // Verify that we can mutate the value at the key
    if let Some(val) = result {
        if let Value::Bool(ref mut b) = *val {
            *b = false;
        }
    }

    // Check that the value was updated correctly
    if let Value::Object(ref map) = value {
        assert_eq!(map.get(&key).unwrap(), &Value::Bool(false));
    }
}

