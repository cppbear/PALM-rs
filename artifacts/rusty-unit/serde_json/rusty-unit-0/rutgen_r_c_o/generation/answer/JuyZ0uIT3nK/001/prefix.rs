// Answer 0

#[test]
fn test_values_mut_empty_map() {
    let mut map = Map::new();
    let _ = map.values_mut();
}

#[test]
fn test_values_mut_non_empty_map() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let _ = map.values_mut();
}

#[test]
fn test_values_mut_with_capacity() {
    let mut map = Map::with_capacity(10);
    map.insert("key1".to_string(), Value::Number(Number::from(42)));
    map.insert("key2".to_string(), Value::String("value".to_string()));
    let _ = map.values_mut();
}

#[test]
fn test_values_mut_large_map() {
    let mut map = Map::new();
    for i in 0..1000 {
        map.insert(format!("key{}", i), Value::String(format!("value{}", i)));
    }
    let _ = map.values_mut();
}

#[test]
fn test_values_mut_after_clear() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.clear();
    let _ = map.values_mut();
}

#[test]
fn test_values_mut_single_entry() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Array(vec![Value::String("item".to_string())]));
    let mut values = map.values_mut();
    let _ = values.next();
}

