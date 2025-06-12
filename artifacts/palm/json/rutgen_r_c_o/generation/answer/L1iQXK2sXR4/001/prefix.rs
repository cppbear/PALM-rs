// Answer 0

#[test]
fn test_remove_existing_key() {
    let mut map = Map::with_capacity(10);
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number::from(42)));
    let value = map.remove(&"key1");
}

#[test]
fn test_remove_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    let value = map.remove(&"key2");
}

#[test]
fn test_remove_after_clear() {
    let mut map = Map::with_capacity(5);
    map.insert("key1".to_string(), Value::String("value".to_string()));
    map.clear();
    let value = map.remove(&"key1");
}

#[test]
fn test_remove_with_capacity_limit() {
    let mut map = Map::with_capacity(100);
    for i in 0..100 {
        map.insert(format!("key{}", i), Value::Number(Number::from(i)));
    }
    let value = map.remove(&"key50");
}

#[test]
fn test_remove_key_boundary_length() {
    let mut map = Map::new();
    let long_key = "a".repeat(256);
    map.insert(long_key.clone(), Value::String("test value".to_string()));
    let value = map.remove(&long_key);
}

#[test]
fn test_remove_key_with_multiple_elements() {
    let mut map = Map::with_capacity(20);
    for i in 0..10 {
        map.insert(format!("key{}", i), Value::Bool(true));
    }
    let value = map.remove(&"key5");
}

#[test]
fn test_remove_empty_map() {
    let mut map = Map::new();
    let value = map.remove(&"nonexistent_key");
}

#[test]
fn test_remove_panic_on_invalid_key_type() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Number(Number::from(1)));
    #[should_panic]
    let value = map.remove(&42); // Invalid key type, should panic
}

