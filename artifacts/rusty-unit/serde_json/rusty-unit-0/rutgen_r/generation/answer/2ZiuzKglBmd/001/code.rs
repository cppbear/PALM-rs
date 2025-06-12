// Answer 0

#[test]
fn test_clear_empty_map() {
    let mut map = serde_json::Map::new();
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_clear_non_empty_map() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), serde_json::Value::from("value1"));
    map.insert("key2".to_string(), serde_json::Value::from("value2"));
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_clear_after_multiple_operations() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), serde_json::Value::from("value1"));
    map.insert("key2".to_string(), serde_json::Value::from("value2"));
    map.remove("key1");
    map.clear();
    assert!(map.is_empty());
}

