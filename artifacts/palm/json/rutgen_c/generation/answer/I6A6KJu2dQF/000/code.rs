// Answer 0

#[test]
fn test_contains_key_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert!(map.contains_key(&"key1"));
}

#[test]
fn test_contains_key_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert!(!map.contains_key(&"key2"));
}

#[test]
fn test_contains_key_empty_map() {
    let map = Map::new();
    assert!(!map.contains_key(&"any_key"));
}

#[test]
fn test_contains_key_with_different_borrowed_form() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let borrowed_key: &str = "key1";
    assert!(map.contains_key(borrowed_key));
}

#[test]
fn test_contains_key_with_case_sensitivity() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert!(!map.contains_key(&"Key1"));
}

