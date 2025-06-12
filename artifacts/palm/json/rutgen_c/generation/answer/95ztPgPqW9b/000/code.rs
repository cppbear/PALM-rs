// Answer 0

#[test]
fn test_map_new() {
    let map: Map<String, Value> = Map::new();
    assert!(map.map.is_empty());
}

#[test]
fn test_map_with_capacity() {
    let capacity = 10;
    let map: Map<String, Value> = Map::with_capacity(capacity);
    assert!(map.map.is_empty());
}

#[test]
fn test_map_new_non_empty() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key".to_string(), Value::Bool(true));
    assert_eq!(map.len(), 1);
    assert!(!map.is_empty());
}

