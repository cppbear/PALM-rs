// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let result = map.get_key_value("key1");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (&"key1".to_string(), &Value::String("value1".to_string())));
}

#[test]
fn test_get_key_value_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let result = map.get_key_value("non_existing_key");
    assert!(result.is_none());
}

#[test]
fn test_get_key_value_empty_map() {
    let map = Map::new();
    
    let result = map.get_key_value("any_key");
    assert!(result.is_none());
}

#[test]
fn test_get_key_value_with_different_borrowed_form() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let key: &str = "key1";
    let result = map.get_key_value(key);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (&"key1".to_string(), &Value::String("value1".to_string())));
}

#[test]
#[should_panic]
fn test_get_key_value_with_incompatible_key_type() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    // This should not panic in practice as it fits the generics but is good for boundary testing.
    let _: Option<&Value> = map.get_key_value(&500);
}

