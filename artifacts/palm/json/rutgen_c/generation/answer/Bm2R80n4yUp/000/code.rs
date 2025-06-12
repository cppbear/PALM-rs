// Answer 0

#[test]
fn test_get_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    assert_eq!(map.get("key1"), Some(&Value::Bool(true)));
}

#[test]
fn test_get_non_existing_key() {
    let map = Map::new();
    assert_eq!(map.get("non_existing_key"), None);
}

#[test]
fn test_get_with_borrowed_key() {
    let mut map = Map::new();
    map.insert("key2".to_string(), Value::Number(42.into()));
    let key: &str = "key2";
    assert_eq!(map.get(key), Some(&Value::Number(42.into())));
}

#[test]
fn test_get_with_different_ordering() {
    let mut map = Map::new();
    map.insert("key3".to_string(), Value::String("value".to_string()));
    // This case does not test a different ordering since a string slice can borrow without order problem
    assert_eq!(map.get("key3"), Some(&Value::String("value".to_string())));
} 

#[test]
fn test_get_with_empty_map() {
    let map: Map<String, Value> = Map::new();
    assert_eq!(map.get("empty_key"), None);
}

