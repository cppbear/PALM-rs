// Answer 0

#[test]
fn test_deserialize_empty_map() {
    let json_data = "{}";
    let deserializer = serde_json::Deserializer::from_str(json_data);
    let result: Result<Map<String, Value>, _> = Map::deserialize(deserializer);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert!(map.map.is_empty());
}

#[test]
fn test_deserialize_single_entry_map() {
    let json_data = r#"{"key":"value"}"#;
    let deserializer = serde_json::Deserializer::from_str(json_data);
    let result: Result<Map<String, Value>, _> = Map::deserialize(deserializer);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert_eq!(map.map.len(), 1);
    assert_eq!(map.map.get("key").unwrap(), &Value::String("value".to_string()));
}

#[test]
fn test_deserialize_multiple_entries_map() {
    let json_data = r#"{"key1": 123, "key2": true, "key3": null}"#;
    let deserializer = serde_json::Deserializer::from_str(json_data);
    let result: Result<Map<String, Value>, _> = Map::deserialize(deserializer);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert_eq!(map.map.len(), 3);
    assert_eq!(map.map.get("key1").unwrap(), &Value::Number(123.into()));
    assert_eq!(map.map.get("key2").unwrap(), &Value::Bool(true));
    assert_eq!(map.map.get("key3").unwrap(), &Value::Null);
}

#[test]
fn test_deserialize_nested_map() {
    let json_data = r#"{"nested": {"key": "value"}}"#;
    let deserializer = serde_json::Deserializer::from_str(json_data);
    let result: Result<Map<String, Value>, _> = Map::deserialize(deserializer);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert_eq!(map.map.len(), 1);
    if let Value::Object(inner_map) = map.map.get("nested").unwrap() {
        assert_eq!(inner_map.map.len(), 1);
        assert_eq!(inner_map.map.get("key").unwrap(), &Value::String("value".to_string()));
    } else {
        panic!("Expected nested map");
    }
}

#[test]
#[should_panic]
fn test_deserialize_invalid_json() {
    let json_data = r#"{key: value}"#; // Invalid JSON, keys must be quoted
    let deserializer = serde_json::Deserializer::from_str(json_data);
    let _result: Result<Map<String, Value>, _> = Map::deserialize(deserializer);
}

