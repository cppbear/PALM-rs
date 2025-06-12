// Answer 0

#[test]
fn test_remove_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let result = map.remove("key1");
    assert_eq!(result, Some(Value::String("value1".to_string())));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let result = map.remove("key2");
    assert_eq!(result, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_remove_key_after_multiple_inserts() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    let result = map.remove("key1");
    assert_eq!(result, Some(Value::String("value1".to_string())));
    assert_eq!(map.len(), 1);
    let result_after = map.remove("key2");
    assert_eq!(result_after, Some(Value::String("value2".to_string())));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_key_when_empty() {
    let mut map = Map::new();
    let result = map.remove("key1");
    assert_eq!(result, None);
    assert_eq!(map.len(), 0);
}

