// Answer 0

#[test]
fn test_iter_empty_map() {
    let map: Map<String, Value> = Map::new();
    let mut iter = map.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let mut iter = map.iter();
    
    assert_eq!(iter.next(), Some((&"key1".to_string(), &Value::String("value1".to_string()))));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    
    let mut iter = map.iter();
    
    assert_eq!(iter.next(), Some((&"key1".to_string(), &Value::String("value1".to_string()))));
    assert_eq!(iter.next(), Some((&"key2".to_string(), &Value::String("value2".to_string()))));
    assert_eq!(iter.next(), None);
}

