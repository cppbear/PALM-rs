// Answer 0

#[test]
fn test_shift_remove_entry_existing_key() {
    #[derive(Hash, PartialEq, Eq)]
    struct TestKey {
        id: i32,
    }

    #[derive(Debug)]
    struct TestValue {
        data: String,
    }

    // Create the IndexMap with appropriate key and value types
    let mut map: IndexMap<TestKey, TestValue, ()> = IndexMap::new();

    // Insert a key-value pair into the map
    let key_to_remove = TestKey { id: 1 };
    let value_to_remove = TestValue { data: "value1".to_string() };
    map.insert(key_to_remove.clone(), value_to_remove);

    // Call the function and store the result
    let result = map.shift_remove_entry(&key_to_remove);

    // Assert that the result is Some((key, value))
    assert_eq!(
        result,
        Some((key_to_remove, TestValue { data: "value1".to_string() }))
    );
}

#[test]
fn test_shift_remove_entry_nonexistent_key() {
    #[derive(Hash, PartialEq, Eq)]
    struct TestKey {
        id: i32,
    }

    #[derive(Debug)]
    struct TestValue {
        data: String,
    }

    // Create the IndexMap with appropriate key and value types
    let mut map: IndexMap<TestKey, TestValue, ()> = IndexMap::new();

    // Insert a key-value pair into the map
    let key_in_map = TestKey { id: 1 };
    let value_in_map = TestValue { data: "value1".to_string() };
    map.insert(key_in_map.clone(), value_in_map);

    // Define a key that does not exist in the map
    let nonexistent_key = TestKey { id: 2 };

    // Call the function and store the result
    let result = map.shift_remove_entry(&nonexistent_key);

    // Assert that the result is None
    assert_eq!(result, None);
}

#[test]
fn test_shift_remove_entry_multiple_elements() {
    #[derive(Hash, PartialEq, Eq)]
    struct TestKey {
        id: i32,
    }

    #[derive(Debug)]
    struct TestValue {
        data: String,
    }

    // Create the IndexMap with appropriate key and value types
    let mut map: IndexMap<TestKey, TestValue, ()> = IndexMap::new();

    // Insert multiple key-value pairs into the map
    let key1 = TestKey { id: 1 };
    let value1 = TestValue { data: "value1".to_string() };
    map.insert(key1.clone(), value1);

    let key2 = TestKey { id: 2 };
    let value2 = TestValue { data: "value2".to_string() };
    map.insert(key2.clone(), value2);

    let key3 = TestKey { id: 3 };
    let value3 = TestValue { data: "value3".to_string() };
    map.insert(key3.clone(), value3);

    // Remove the second key and check the result
    let result = map.shift_remove_entry(&key2);

    // Assert that the result is Some((key2, value2))
    assert_eq!(
        result,
        Some((key2, TestValue { data: "value2".to_string() }))
    );

    // Verify that key3 is still accessible
    let remaining_value = map.get(&key3);
    assert!(remaining_value.is_some());
    assert_eq!(remaining_value.unwrap().data, "value3");

    // Verify that key1 is still in the map
    let remaining_value = map.get(&key1);
    assert!(remaining_value.is_some());
    assert_eq!(remaining_value.unwrap().data, "value1");
}

