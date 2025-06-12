// Answer 0

#[test]
fn test_retain_with_all_elements_retained() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Bool(true));

    map.retain(|_, _| true);

    assert_eq!(map.len(), 2);
}

#[test]
fn test_retain_with_no_elements_retained() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Bool(true));

    map.retain(|_, _| false);

    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_with_some_elements_retained() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Bool(true));
    map.insert("key3".to_string(), Value::String("test".to_string()));

    map.retain(|key, _| key == "key2");

    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&"key2".to_string()));
    assert!(!map.contains_key(&"key1".to_string()));
    assert!(!map.contains_key(&"key3".to_string()));
}

#[test]
fn test_retain_with_empty_map() {
    let mut map = Map::new();

    map.retain(|_, _| false);

    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_with_complex_values() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Array(vec![Value::String("a".to_string()), Value::Number(Number::from(1))]));
    map.insert("key2".to_string(), Value::Object(Map::new()));

    map.retain(|key, value| {
        match value {
            Value::Array(_) => key == "key1",
            _ => false,
        }
    });

    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&"key1".to_string()));
    assert!(!map.contains_key(&"key2".to_string()));
}

