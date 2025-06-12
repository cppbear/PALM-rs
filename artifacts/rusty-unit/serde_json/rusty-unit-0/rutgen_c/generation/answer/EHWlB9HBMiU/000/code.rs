// Answer 0

#[test]
fn test_retain_removes_elements() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Bool(false));
    map.insert("key3".to_string(), Value::Bool(true));

    map.retain(|_k, v| {
        if let Value::Bool(b) = v {
            *b // Retain only if the value is true
        } else {
            false
        }
    });

    assert_eq!(map.len(), 2);
    assert!(map.contains_key("key1"));
    assert!(map.contains_key("key3"));
    assert!(!map.contains_key("key2"));
}

#[test]
fn test_retain_removes_all_elements() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(false));
    map.insert("key2".to_string(), Value::Bool(false));

    map.retain(|_k, v| {
        if let Value::Bool(b) = v {
            *b // Retain only if the value is true
        } else {
            false
        }
    });

    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_no_elements_removed() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Bool(true));

    map.retain(|_k, v| {
        if let Value::Bool(b) = v {
            *b // Retain only if the value is true
        } else {
            false
        }
    });

    assert_eq!(map.len(), 2);
}

#[test]
fn test_retain_empty_map() {
    let mut map: Map<String, Value> = Map::new();

    map.retain(|_k, _v| true); // No change should occur

    assert_eq!(map.len(), 0);
}

