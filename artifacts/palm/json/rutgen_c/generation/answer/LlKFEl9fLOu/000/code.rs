// Answer 0

#[test]
fn test_index_mut_existing_key() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    if let Value::String(ref mut value) = map.index_mut(&"key1") {
        *value = "updated_value1".to_string();
    }
    
    assert_eq!(map.map.get("key1"), Some(&Value::String("updated_value1".to_string())));
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_non_existing_key() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.index_mut(&"non_existing_key");
}

