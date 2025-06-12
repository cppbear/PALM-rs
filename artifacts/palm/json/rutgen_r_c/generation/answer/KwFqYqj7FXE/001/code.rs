// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let result = map.remove_entry("key1");
    assert_eq!(result, Some(("key1".to_string(), Value::String("value1".to_string()))));
    assert!(map.is_empty());
}

#[test]
fn test_remove_entry_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));

    let result = map.remove_entry("key2");
    assert_eq!(result, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_remove_entry_multiple_keys() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));

    let result = map.remove_entry("key1");
    assert_eq!(result, Some(("key1".to_string(), Value::String("value1".to_string()))));
    assert_eq!(map.len(), 1);
    assert!(map.get("key1").is_none());
    assert_eq!(map.get("key2"), Some(&Value::String("value2".to_string())));
}

#[test]
fn test_remove_entry_empty_map() {
    let mut map: Map<String, Value> = Map::new();
    
    let result = map.remove_entry("key1");
    assert_eq!(result, None);
    assert!(map.is_empty());
}

#[test]
#[should_panic]
fn test_remove_entry_panic_condition() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));

    // Attempt to remove entry with key borrowed as integer (invalid type)
    let _ = map.remove_entry(&1);
}

