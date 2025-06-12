// Answer 0

#[test]
fn test_keys_empty_map() {
    let map: Map<String, Value> = Map::new();
    let keys = map.keys();
    assert!(keys.iter.len() == 0);
}

#[test]
fn test_keys_single_element() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let keys = map.keys();
    let key_vec: Vec<String> = keys.iter.collect();
    assert_eq!(key_vec, vec!["key1"]);
}

#[test]
fn test_keys_multiple_elements() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    map.insert("key3".to_string(), Value::String("value3".to_string()));
    let keys = map.keys();
    let key_vec: Vec<String> = keys.iter.collect();
    assert_eq!(key_vec, vec!["key1", "key2", "key3"]);
}

#[test]
fn test_keys_after_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.clear();
    let keys = map.keys();
    assert!(keys.iter.len() == 0);
}

#[test]
fn test_keys_order_of_insertion() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    map.insert("key3".to_string(), Value::String("value3".to_string()));
    let keys = map.keys();
    let key_vec: Vec<String> = keys.iter.collect();
    assert_eq!(key_vec, vec!["key1", "key2", "key3"]);
}

