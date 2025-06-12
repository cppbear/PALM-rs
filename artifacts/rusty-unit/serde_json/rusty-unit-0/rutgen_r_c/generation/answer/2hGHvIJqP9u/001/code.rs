// Answer 0

#[test]
fn test_is_empty_new_map() {
    let map: Map<String, Value> = Map::new();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_with_capacity() {
    let map: Map<String, Value> = Map::with_capacity(10);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_non_empty_map() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key".to_string(), Value::Bool(true));
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key".to_string(), Value::Bool(true));
    map.clear();
    assert!(map.is_empty());
}

