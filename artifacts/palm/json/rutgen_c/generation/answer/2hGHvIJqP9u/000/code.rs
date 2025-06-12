// Answer 0

#[test]
fn test_map_is_empty_initially() {
    let map: Map<String, Value> = Map::new();
    assert!(map.is_empty());
}

#[test]
fn test_map_is_empty_after_insert() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Null);
    assert!(!map.is_empty());
}

#[test]
fn test_map_is_empty_after_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_map_is_empty_after_multiple_inserts_and_clears() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Bool(true));
    assert!(!map.is_empty());
    map.clear();
    assert!(map.is_empty());
}

