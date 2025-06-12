// Answer 0

#[test]
fn test_len_empty_map() {
    let map: Map<String, Value> = Map::new();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_non_empty_map() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(10)));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_len_after_removal() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    map.remove("key1");
    assert_eq!(map.len(), 1);
}

#[test]
fn test_len_after_clear() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.clear();
    assert_eq!(map.len(), 0);
}

