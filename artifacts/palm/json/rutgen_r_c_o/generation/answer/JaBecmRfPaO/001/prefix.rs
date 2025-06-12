// Answer 0

#[test]
fn test_get_mut_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let value = map.get_mut("key1");
}

#[test]
fn test_get_mut_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(false));
    let value = map.get_mut("key2");
}

#[test]
fn test_get_mut_empty_map() {
    let mut map = Map::new();
    let value = map.get_mut("key1");
}

#[test]
fn test_get_mut_with_large_key() {
    let mut map = Map::with_capacity(10);
    map.insert("key_large".to_string(), Value::Number(500.0));
    let value = map.get_mut("key_large");
}

#[test]
fn test_get_mut_boundary_key_length() {
    let mut map = Map::new();
    let long_key = "a".repeat(256); // Max length key
    map.insert(long_key.clone(), Value::String("max length key".to_string()));
    let value = map.get_mut(&long_key);
}

#[test]
fn test_get_mut_edge_value_object() {
    let mut map = Map::new();
    let mut obj_map = Map::new();
    obj_map.insert("inner_key".to_string(), Value::String("inner_value".to_string()));
    map.insert("outer_key".to_string(), Value::Object(obj_map));
    let value = map.get_mut("outer_key");
}

#[test]
fn test_get_mut_edge_value_array() {
    let mut map = Map::new();
    map.insert("array_key".to_string(), Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]));
    let value = map.get_mut("array_key");
}

#[test]
fn test_get_mut_modify_value() {
    let mut map = Map::new();
    map.insert("changeable_key".to_string(), Value::Number(10.0));
    if let Some(value) = map.get_mut("changeable_key") {
        *value = Value::Number(20.0);
    }
}

