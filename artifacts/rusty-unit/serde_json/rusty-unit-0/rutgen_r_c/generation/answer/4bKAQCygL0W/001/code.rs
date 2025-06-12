// Answer 0

#[test]
fn test_insert_new_key() {
    let mut map = Map::new();
    let result = map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert_eq!(result, None);
}

#[test]
fn test_insert_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let result = map.insert("key1".to_string(), Value::String("value2".to_string()));
    assert_eq!(result, Some(Value::String("value1".to_string())));
}

#[test]
fn test_insert_multiple_keys() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(42)));
    let result1 = map.insert("key1".to_string(), Value::Bool(true));
    let result2 = map.insert("key3".to_string(), Value::Array(vec![Value::Null]));
    
    assert_eq!(result1, Some(Value::String("value1".to_string())));
    assert_eq!(result2, None);
}

#[test]
fn test_insert_with_complex_value() {
    let mut map = Map::new();
    let complex_value = Value::Object(Map::new().insert("nested_key".to_string(), Value::Number(serde_json::Number::from(3))));
    let result = map.insert("complex".to_string(), complex_value);
    assert_eq!(result, None);
}

#[test]
fn test_insert_empty_key() {
    let mut map = Map::new();
    let result = map.insert("".to_string(), Value::String("empty_key_value".to_string()));
    assert_eq!(result, None);
}

#[test]
fn test_insert_empty_value() {
    let mut map = Map::new();
    let result = map.insert("key_with_empty_value".to_string(), Value::Null);
    assert_eq!(result, None);
}

