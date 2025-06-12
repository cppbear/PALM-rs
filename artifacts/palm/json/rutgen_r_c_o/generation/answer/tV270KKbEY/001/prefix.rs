// Answer 0

#[test]
fn test_into_values_empty_map() {
    let map: Map<String, Value> = Map::new();
    let values_iter = map.into_values();
}

#[test]
fn test_into_values_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let values_iter = map.into_values();
}

#[test]
fn test_into_values_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    for i in 0..10 {
        map.insert(format!("key{}", i), Value::Number(i.into()));
    }
    let values_iter = map.into_values();
}

#[test]
fn test_into_values_with_large_capacity() {
    let mut map: Map<String, Value> = Map::with_capacity(1000);
    for i in 0..1000 {
        map.insert(format!("key{}", i), Value::String(format!("value{}", i)));
    }
    let values_iter = map.into_values();
}

#[test]
fn test_into_values_various_value_lengths() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("short".to_string(), Value::String("a".to_string()));
    map.insert("medium".to_string(), Value::String("medium_value".to_string()));
    map.insert("long".to_string(), Value::String("x".repeat(1024)));
    let values_iter = map.into_values();
}

#[test]
fn test_into_values_capacity_edge_case() {
    let mut map: Map<String, Value> = Map::with_capacity(1);
    map.insert("key1".to_string(), Value::Null);
    let values_iter = map.into_values();
}

