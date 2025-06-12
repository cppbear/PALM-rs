// Answer 0

#[test]
fn test_get_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    assert_eq!(map.get(&"key1"), Some(&Value::String("value1".to_string())));
}

#[test]
fn test_get_non_existing_key() {
    let map = Map::new();
    assert_eq!(map.get(&"non_existing_key"), None);
}

#[test]
fn test_get_key_with_different_form() {
    let mut map = Map::new();
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    assert_eq!(map.get(&"key2"), Some(&Value::String("value2".to_string())));
    assert_eq!(map.get(&String::from("key2")), Some(&Value::String("value2".to_string())));
}

#[test]
fn test_get_empty_map() {
    let map = Map::new();
    assert_eq!(map.get(&"any_key"), None);
}

#[test]
fn test_get_with_prefix() {
    let mut map = Map::new();
    map.insert("prefix_key".to_string(), Value::String("value3".to_string()));
    assert_eq!(map.get(&"prefix_key"), Some(&Value::String("value3".to_string())));
    assert_eq!(map.get(&"prefix_key_other"), None);
}

