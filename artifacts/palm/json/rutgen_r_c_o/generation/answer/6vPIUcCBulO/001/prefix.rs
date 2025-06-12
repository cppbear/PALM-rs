// Answer 0

#[test]
fn test_index_valid_key() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("key1".to_string(), Value::Number(Number::from(1)));
    let result = map.index(&"key1");
}

#[test]
#[should_panic]
fn test_index_nonexistent_key() {
    let map = Map { map: MapImpl::new() };
    let result = map.index(&"nonexistent_key");
}

#[test]
fn test_index_multiple_entries() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("key1".to_string(), Value::Number(Number::from(1)));
    map.map.insert("key2".to_string(), Value::Bool(true));
    map.map.insert("key3".to_string(), Value::String("test".to_string()));
    
    let result1 = map.index(&"key1");
    let result2 = map.index(&"key2");
    let result3 = map.index(&"key3");
}

#[test]
fn test_index_edge_case_zero() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("0".to_string(), Value::Number(Number::from(0)));
    let result = map.index(&"0");
}

#[test]
fn test_index_edge_case_large_number() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("1000".to_string(), Value::Number(Number::from(1000)));
    let result = map.index(&"1000");
}

#[test]
#[should_panic]
fn test_index_edge_case_negative() {
    let map = Map { map: MapImpl::new() };
    let result = map.index(&"-1");
}

