// Answer 0

#[test]
fn test_index_existing_key() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    let value = &map["key1"];
    assert_eq!(value, &Value::String("value1".to_string()));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_non_existing_key() {
    let map = Map {
        map: MapImpl::new(),
    };
    let _value = &map["non_existing_key"];
}

#[test]
fn test_index_empty_map() {
    let map = Map {
        map: MapImpl::new(),
    };
    assert!(map.map.is_empty());
}

#[test]
fn test_index_multiple_entries() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.map.insert("key2".to_string(), Value::Number(Number::from(42)));
    
    assert_eq!(map["key1"], Value::String("value1".to_string()));
    assert_eq!(map["key2"], Value::Number(Number::from(42)));
}

