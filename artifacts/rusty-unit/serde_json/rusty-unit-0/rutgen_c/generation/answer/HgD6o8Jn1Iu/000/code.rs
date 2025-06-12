// Answer 0

#[test]
fn test_iter_mut_empty() {
    let mut map = Map::new();
    let mut iter = map.iter_mut();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut_single_entry() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let mut iter = map.iter_mut();
    assert_eq!(iter.next().unwrap(), (&"key1".to_string(), &mut Value::Bool(true)));
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Bool(false));
    let mut iter = map.iter_mut();

    let entry1 = iter.next().unwrap();
    let entry2 = iter.next().unwrap();

    assert!(entry1.0 == &"key1".to_string() || entry1.0 == &"key2".to_string());
    assert!(entry2.0 == &"key1".to_string() || entry2.0 == &"key2".to_string());
    assert!(entry1.0 != entry2.0);
}

#[test]
fn test_iter_mut_update_value() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Number(serde_json::Number::from(1)));
    {
        let mut iter = map.iter_mut();
        if let Some((_, value)) = iter.next() {
            *value = Value::Number(serde_json::Number::from(2));
        }
    }
    assert_eq!(map.get("key1"), Some(&Value::Number(serde_json::Number::from(2))));
}

