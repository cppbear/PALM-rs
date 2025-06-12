// Answer 0

#[test]
fn test_values_empty_map() {
    let map: Map<String, Value> = Map::new();
    let _ = map.values();
}

#[test]
fn test_values_single_entry() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    let _ = map.values();
}

#[test]
fn test_values_multiple_entries() {
    let mut map = Map::new();
    for i in 0..10 {
        map.insert(format!("key{}", i), Value::Number(i.into()));
    }
    let _ = map.values();
}

#[test]
fn test_values_with_capacity() {
    let mut map = Map::with_capacity(100);
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]));
    let _ = map.values();
}

#[test]
fn test_values_large_map() {
    let mut map = Map::new();
    for i in 0..1000 {
        map.insert(format!("key{}", i), Value::String(format!("value{}", i)));
    }
    let _ = map.values();
}

#[test]
fn test_values_edges() {
    let mut map = Map::new();
    let empty_key = "".to_string();
    map.insert(empty_key.clone(), Value::Null);
    map.insert("normal_key".to_string(), Value::Bool(false));
    let _ = map.values();
}

