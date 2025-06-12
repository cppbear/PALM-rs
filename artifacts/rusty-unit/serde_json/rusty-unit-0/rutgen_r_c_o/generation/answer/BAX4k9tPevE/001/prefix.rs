// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let result = map.get_key_value("key1");
}

#[test]
fn test_get_key_value_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let result = map.get_key_value("key2");
}

#[test]
fn test_get_key_value_bool() {
    let mut map = Map::new();
    map.insert("key_bool".to_string(), Value::Bool(false));
    let result = map.get_key_value("key_bool");
}

#[test]
fn test_get_key_value_null() {
    let mut map = Map::new();
    map.insert("key_null".to_string(), Value::Null);
    let result = map.get_key_value("key_null");
}

#[test]
fn test_get_key_value_number() {
    let mut map = Map::new();
    map.insert("key_number".to_string(), Value::Number(42.into()));
    let result = map.get_key_value("key_number");
}

#[test]
fn test_get_key_value_string() {
    let mut map = Map::new();
    map.insert("key_string".to_string(), Value::String("A non-empty string".to_string()));
    let result = map.get_key_value("key_string");
}

#[test]
fn test_get_key_value_array() {
    let mut map = Map::new();
    map.insert("key_array".to_string(), Value::Array(vec![Value::Bool(true), Value::Number(3.14.into())]));
    let result = map.get_key_value("key_array");
}

#[test]
fn test_get_key_value_object() {
    let mut map_inner = Map::new();
    map_inner.insert("inner_key".to_string(), Value::String("inner_value".to_string()));
    
    let mut map = Map::new();
    map.insert("key_object".to_string(), Value::Object(map_inner));
    
    let result = map.get_key_value("key_object");
}

#[test]
fn test_get_key_value_edge_capacity() {
    let mut map = Map::with_capacity(999);
    for i in 0..999 {
        map.insert(format!("key_{}", i), Value::Bool(i % 2 == 0));
    }
    let result = map.get_key_value("key_999"); // should return None
}

