// Answer 0

#[test]
fn test_index_existing_key() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.map.insert("key2".to_string(), Value::Bool(true));

    // Accessing existing key
    let value = &map["key1"];
    assert_eq!(value, &Value::String("value1".to_string()));
}

#[test]
#[should_panic(expected = "key not found")]
fn test_index_non_existing_key() {
    let map = Map { map: MapImpl::new() };
    
    // Attempting to access a non-existing key should panic
    let _value = &map["non_existing_key"];
}

#[test]
fn test_index_empty_map() {
    let map = Map { map: MapImpl::new() };

    // Accessing a key in an empty map should panic
    // Since we don't expect any specific message for empty check, we're focusing on the panic.
    let _value = &map["any_key"]; // This should panic
}

#[test]
fn test_index_multiple_entries() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("key1".to_string(), Value::Number(Number::from_f64(3.14).unwrap()));
    map.map.insert("key2".to_string(), Value::Array(vec![Value::String("item1".to_string())]));

    // Accessing multiple existing keys
    let value1 = &map["key1"];
    let value2 = &map["key2"];
    
    assert_eq!(value1, &Value::Number(Number::from_f64(3.14).unwrap()));
    assert_eq!(value2, &Value::Array(vec![Value::String("item1".to_string())]));
}

