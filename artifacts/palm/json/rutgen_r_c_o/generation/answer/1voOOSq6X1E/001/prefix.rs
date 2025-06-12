// Answer 0

#[test]
fn test_append_empty_maps() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    map1.append(&mut map2);
}

#[test]
fn test_append_non_empty_map_to_empty_map() {
    let mut map1 = Map::new();
    let mut map2 = Map::with_capacity(10);
    map2.insert("key1".to_string(), Value::Bool(true));
    map2.insert("key2".to_string(), Value::Number(Value::Number(3.14)));
    map1.append(&mut map2);
}

#[test]
fn test_append_empty_map_to_non_empty_map() {
    let mut map1 = Map::with_capacity(10);
    map1.insert("key1".to_string(), Value::String("value1".to_string()));
    let mut map2 = Map::new();
    map1.append(&mut map2);
}

#[test]
fn test_append_non_empty_map_to_non_empty_map() {
    let mut map1 = Map::with_capacity(10);
    map1.insert("key1".to_string(), Value::Bool(false));
    let mut map2 = Map::with_capacity(5);
    map2.insert("key2".to_string(), Value::Null);
    map2.insert("key3".to_string(), Value::Array(vec![Value::String("a".to_string()), Value::String("b".to_string())]));
    map1.append(&mut map2);
}

#[test]
fn test_append_large_maps() {
    let mut map1 = Map::with_capacity(1000);
    for i in 0..1000 {
        map1.insert(format!("key{}", i), Value::Number(Value::Number(i as f64)));
    }
    let mut map2 = Map::with_capacity(1000);
    for i in 0..1000 {
        map2.insert(format!("key{}", i + 1000), Value::String(format!("value{}", i)));
    }
    map1.append(&mut map2);
}

#[test]
fn test_append_with_keys_and_values_length_limits() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    for i in 0..100 {
        let key = format!("key{:02}", i);
        let value = Value::String("value".repeat(100)); // 100 characters long
        map2.insert(key, value);
    }
    map1.append(&mut map2);
}

#[test]
#[should_panic]
fn test_append_with_overflow_capacity() {
    let mut map1 = Map::with_capacity(1000);
    let mut map2 = Map::with_capacity(1000);
    for i in 0..1000 {
        map1.insert(format!("key1-{}", i), Value::Null);
        map2.insert(format!("key2-{}", i), Value::Null);
    }
    map1.append(&mut map2);
    // This should panic as map1 will exceed its initial capacity.
}

#[test]
fn test_append_remains_empty_if_other_empty() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    map1.append(&mut map2);
    assert!(map1.is_empty());
}

