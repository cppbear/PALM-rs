// Answer 0

#[test]
fn test_contains_key_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    assert!(map.contains_key("key1"));
}

#[test]
fn test_contains_key_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    assert!(!map.contains_key("key2"));
}

#[test]
fn test_contains_key_empty_map() {
    let map = Map::new();
    assert!(!map.contains_key("any_key"));
}

#[test]
fn test_contains_key_with_borrowed_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let borrowed_key: &str = "key1";
    assert!(map.contains_key(borrowed_key));
}

#[test]
fn test_contains_key_with_different_borrowed_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let borrowed_key: &str = "key2";
    assert!(!map.contains_key(borrowed_key));
}

