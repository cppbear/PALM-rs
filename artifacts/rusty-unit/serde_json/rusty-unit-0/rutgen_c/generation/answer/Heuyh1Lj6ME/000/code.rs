// Answer 0

#[test]
fn test_len_empty_map() {
    let map = Map::<String, Value>::new();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_with_capacity() {
    let mut map = Map::with_capacity(10);
    assert_eq!(map.len(), 0);
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_len_after_insertions() {
    let mut map = Map::<String, Value>::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_len_after_removals() {
    let mut map = Map::<String, Value>::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    map.remove(&"key1".to_string());
    assert_eq!(map.len(), 1);
    map.remove(&"key2".to_string());
    assert_eq!(map.len(), 0);
}

