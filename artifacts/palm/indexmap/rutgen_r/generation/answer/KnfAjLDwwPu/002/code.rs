// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    // Define a simple key-value struct
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct KeyType(i32);

    // Create a new IndexMap and populate it with a key-value pair
    let mut map = IndexMap::new();
    let key = KeyType(10);
    let value = "ten";

    map.insert(key.clone(), value);

    // Test the get_key_value function
    let result = map.get_key_value(&key);
    assert!(result.is_some());

    if let Some((k, v)) = result {
        assert_eq!(k, &key);
        assert_eq!(v, &value);
    } else {
        panic!("Expected Some value but got None");
    }
}

#[test]
fn test_get_key_value_non_existent_key() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct KeyType(i32);

    let map = IndexMap::new();
    let key = KeyType(20);  // A key that does not exist in the map

    // Test the get_key_value function
    let result = map.get_key_value(&key);
    assert!(result.is_none());
}

