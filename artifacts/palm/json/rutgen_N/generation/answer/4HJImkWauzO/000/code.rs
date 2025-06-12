// Answer 0

#[test]
fn test_new_creates_map_deserializer() {
    use serde_json::{Map, Value};
    
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(42)));
    
    let deserializer = new(map);
    
    assert!(deserializer.iter.len() == 2);
    assert!(deserializer.value.is_none());
}

