// Answer 0

#[test]
fn test_into_values_empty_map() {
    let map: Map<String, Value> = Map::new();
    let values_iter = map.into_values();
    let values: Vec<Value> = values_iter.iter.collect();
    assert_eq!(values.len(), 0);
}

#[test]
fn test_into_values_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let values_iter = map.into_values();
    let values: Vec<Value> = values_iter.iter.collect();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0], Value::Bool(true));
}

#[test]
fn test_into_values_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(42.into()));
    let values_iter = map.into_values();
    let values: Vec<Value> = values_iter.iter.collect();
    assert_eq!(values.len(), 2);
    assert!(values.contains(&Value::Bool(true)));
    assert!(values.contains(&Value::Number(42.into())));
}

#[test]
fn test_into_values_order_preservation() {
    #[cfg(feature = "preserve_order")]
    {
        let mut map: Map<String, Value> = Map::with_capacity(2);
        map.insert("key1".to_string(), Value::Bool(true));
        map.insert("key2".to_string(), Value::Number(42.into()));
        let values_iter = map.into_values();
        let values: Vec<Value> = values_iter.iter.collect();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], Value::Bool(true));
        assert_eq!(values[1], Value::Number(42.into()));
    }
}

