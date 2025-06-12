// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let result = map.get_key_value(&"key1");
    assert_eq!(result, Some((&"key1".to_string(), &Value::String("value1".to_string()))));
}

#[test]
fn test_get_key_value_non_existing_key() {
    let map = Map::new();
    let result = map.get_key_value(&"non_existing");
    assert_eq!(result, None);
}

#[test]
fn test_get_key_value_with_different_borrowed_form() {
    let mut map = Map::new();
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    let result = map.get_key_value(&String::from("key2"));
    assert_eq!(result, Some((&"key2".to_string(), &Value::String("value2".to_string()))));
}

#[test]
fn test_get_key_value_with_empty_map() {
    let map = Map::new();
    let result = map.get_key_value(&"empty_key");
    assert_eq!(result, None);
}

