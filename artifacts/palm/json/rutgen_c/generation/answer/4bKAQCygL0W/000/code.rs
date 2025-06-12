// Answer 0

#[test]
fn test_insert_new_key() {
    let mut map = Map::new();
    let result = map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert_eq!(result, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_insert_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let result = map.insert("key1".to_string(), Value::String("value2".to_string()));
    assert_eq!(result, Some(Value::String("value1".to_string())));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_insert_empty_map() {
    let mut map = Map::new();
    assert!(map.is_empty());
    let result = map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert_eq!(result, None);
    assert!(!map.is_empty());
}

#[test]
fn test_insert_multiple_keys() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    assert_eq!(map.len(), 2);
    assert_eq!(map.insert("key1".to_string(), Value::String("new_value1".to_string())), Some(Value::String("value1".to_string())));
    assert_eq!(map.len(), 2);
}

