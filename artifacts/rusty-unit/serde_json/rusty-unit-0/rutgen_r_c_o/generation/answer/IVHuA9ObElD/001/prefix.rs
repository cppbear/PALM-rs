// Answer 0

#[test]
fn test_into_deserializer_empty_map() {
    let map = Map { map: MapImpl::new() };
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_single_entry() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_multiple_entries() {
    let mut map = Map { map: MapImpl::new() };
    for i in 0..10 {
        map.map.insert(format!("key{}", i), Value::String(format!("value{}", i)));
    }
    let _deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_large_map() {
    let mut map = Map { map: MapImpl::new() };
    for i in 0..10000 {
        map.map.insert(format!("key{}", i), Value::String(format!("value{}", i)));
    }
    let _deserializer = map.into_deserializer();
}

