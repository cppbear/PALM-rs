// Answer 0

#[test]
fn test_values_empty() {
    let map: Map<String, Value> = Map::new();
    let values_iter = map.values();
    let values: Vec<&Value> = values_iter.iter.collect();
    assert!(values.is_empty());
}

#[test]
fn test_values_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Number(42.into()));
    let values_iter = map.values();
    let values: Vec<&Value> = values_iter.iter.collect();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0], &Value::Number(42.into()));
}

#[test]
fn test_values_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Bool(true));
    let values_iter = map.values();
    let values: Vec<&Value> = values_iter.iter.collect();
    assert_eq!(values.len(), 2);
    assert!(values.contains(&&Value::String("value1".to_string())));
    assert!(values.contains(&&Value::Bool(true)));
}

#[test]
fn test_values_map_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Number(10.into()));
    map.clear();
    let values_iter = map.values();
    let values: Vec<&Value> = values_iter.iter.collect();
    assert!(values.is_empty());
}

