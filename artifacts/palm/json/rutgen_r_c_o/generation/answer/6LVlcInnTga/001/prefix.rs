// Answer 0

#[test]
fn test_iter_empty_map() {
    let map: Map<String, Value> = Map::new();
    let _ = map.iter();
}

#[test]
fn test_iter_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let _ = map.iter();
}

#[test]
fn test_iter_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    for i in 0..10 {
        map.insert(format!("key{}", i), Value::Number(Number::from(i)));
    }
    let _ = map.iter();
}

#[test]
fn test_iter_with_capacity() {
    let mut map: Map<String, Value> = Map::with_capacity(100);
    for i in 0..50 {
        map.insert(format!("key{}", i), Value::String("value".to_string()));
    }
    let _ = map.iter();
}

#[test]
fn test_iter_large_capacity() {
    let mut map: Map<String, Value> = Map::with_capacity(1000);
    for i in 0..1000 {
        map.insert(format!("key{}", i), Value::Array(vec![Value::Null]));
    }
    let _ = map.iter();
}

