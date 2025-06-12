// Answer 0

#[test]
fn test_iter_empty_map() {
    let map: Map<String, Value> = Map::new();
    let mut iterator = map.iter();
    
    assert_eq!(iterator.len(), 0);
    assert!(iterator.next().is_none());
}

#[test]
fn test_iter_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let mut iterator = map.iter();
    
    assert_eq!(iterator.len(), 1);
    if let Some((key, value)) = iterator.next() {
        assert_eq!(key, "key1");
        assert_eq!(value, &Value::String("value1".to_string()));
    } else {
        panic!("Expected to get an entry, but got None");
    }
}

#[test]
fn test_iter_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(Number::from(2)));
    
    let mut iterator = map.iter();
    assert_eq!(iterator.len(), 2);
    
    let entry1 = iterator.next().expect("Expected first entry");
    assert!(["key1", "key2"].contains(&entry1.0));
    
    let entry2 = iterator.next().expect("Expected second entry");
    assert!(["key1", "key2"].contains(&entry2.0));
    assert_ne!(entry1.0, entry2.0);
    assert!(iterator.next().is_none());
}

#[test]
fn test_iter_after_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.clear();
    
    let mut iterator = map.iter();
    assert_eq!(iterator.len(), 0);
    assert!(iterator.next().is_none());
}

