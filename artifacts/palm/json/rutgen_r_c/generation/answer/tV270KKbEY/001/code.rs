// Answer 0

#[test]
fn test_into_values_empty() {
    let map = Map::new();
    let values = map.into_values();
    assert_eq!(values.iter.len(), 0);
}

#[test]
fn test_into_values_single_entry() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let values = map.into_values();
    assert_eq!(values.iter.len(), 1);
    assert_eq!(values.iter.next().unwrap(), Value::Bool(true));
}

#[test]
fn test_into_values_multiple_entries() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number::from(42)));
    map.insert("key3".to_string(), Value::String("value".to_string()));
    
    let values = map.into_values();
    let mut collected_values: Vec<Value> = values.iter.collect();
    
    assert_eq!(collected_values.len(), 3);
    assert!(collected_values.contains(&Value::Bool(true)));
    assert!(collected_values.contains(&Value::Number(Number::from(42))));
    assert!(collected_values.contains(&Value::String("value".to_string())));
}

#[test]
fn test_into_values_after_clearing() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.clear();
    let values = map.into_values();
    assert_eq!(values.iter.len(), 0);
}

