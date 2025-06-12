// Answer 0

#[test]
fn test_into_deserializer() {
    let map = Map {
        map: MapImpl::new(),
    };
    let deserializer = map.into_deserializer();
    assert_eq!(deserializer.map.len(), 0);
}

#[test]
fn test_into_deserializer_with_items() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.map.insert("key2".to_string(), Value::Number(serde_json::Number::from(42)));
    
    let deserializer = map.into_deserializer();
    assert_eq!(deserializer.map.len(), 2);
    assert!(deserializer.map.contains_key("key1"));
    assert!(deserializer.map.contains_key("key2"));
}

#[test]
fn test_into_deserializer_empty() {
    let map = Map {
        map: MapImpl::new(),
    };
    let deserializer = map.into_deserializer();
    assert_eq!(deserializer.map.len(), 0);
}

