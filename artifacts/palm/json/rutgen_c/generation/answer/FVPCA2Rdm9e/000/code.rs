// Answer 0

#[test]
fn test_keys_empty_map() {
    let map: Map<String, Value> = Map::new();
    let keys_iter = map.keys();
    assert_eq!(keys_iter.iter.len(), 0);
}

#[test]
fn test_keys_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let mut keys_iter = map.keys();
    assert_eq!(keys_iter.iter.len(), 1);
    assert_eq!(keys_iter.iter.next(), Some(&"key1".to_string()));
}

#[test]
fn test_keys_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    let mut keys_iter = map.keys();
    assert_eq!(keys_iter.iter.len(), 2);
    let first_key = keys_iter.iter.next();
    let second_key = keys_iter.iter.next();
    assert!(first_key == Some(&"key1".to_string()) || first_key == Some(&"key2".to_string()));
    assert!(second_key == Some(&"key1".to_string()) || second_key == Some(&"key2".to_string()));
}

#[test]
fn test_keys_after_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.clear();
    let keys_iter = map.keys();
    assert_eq!(keys_iter.iter.len(), 0);
}

