// Answer 0

#[test]
fn test_values_empty_map() {
    let map: Map<String, Value> = Map::new();
    let values = map.values();
    let mut iter = values.iter;
    assert_eq!(iter.len(), 0);
}

#[test]
fn test_values_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let values = map.values();
    let mut iter = values.iter;
    
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some(&Value::String("value1".to_string())));
}

#[test]
fn test_values_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(Number::from(12.5)));
    let values = map.values();
    let mut iter = values.iter;

    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(&Value::String("value1".to_string())));
    assert_eq!(iter.next(), Some(&Value::Number(Number::from(12.5))));
}

#[test]
fn test_values_after_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.clear();
    let values = map.values();
    let mut iter = values.iter;

    assert_eq!(iter.len(), 0);
}

#[test]
fn test_values_iterable() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Bool(true));
    let values = map.values();
    
    let collected: Vec<&Value> = values.iter.collect();
    assert_eq!(collected.len(), 2);
    assert!(collected.contains(&&Value::String("value1".to_string())));
    assert!(collected.contains(&&Value::Bool(true)));
}

