// Answer 0

#[test]
fn test_clear_empty_map() {
    let mut map = Map::new();
    assert!(map.is_empty());
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_clear_non_empty_map() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Null);
    assert!(!map.is_empty());
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_clear_after_insertion() {
    let mut map = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));
    assert_eq!(map.len(), 1);
    map.clear();
    assert_eq!(map.len(), 0);
}

