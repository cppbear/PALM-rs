// Answer 0

#[test]
fn test_deserialize_empty_map() {
    let json_data = "{}";
    let result: Result<Map<String, Value>, _> = serde_json::from_str(json_data);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert!(map.is_empty());
}

#[test]
fn test_deserialize_single_entry_map() {
    let json_data = r#"{"key": 1}"#;
    let result: Result<Map<String, Value>, _> = serde_json::from_str(json_data);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key"), Some(&Value::from(1)));
}

#[test]
fn test_deserialize_multiple_entries_map() {
    let json_data = r#"{"key1": 1, "key2": 2, "key3": 3}"#;
    let result: Result<Map<String, Value>, _> = serde_json::from_str(json_data);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("key1"), Some(&Value::from(1)));
    assert_eq!(map.get("key2"), Some(&Value::from(2)));
    assert_eq!(map.get("key3"), Some(&Value::from(3)));
}

#[test]
fn test_deserialize_null_map() {
    let json_data = "null";
    let result: Result<Map<String, Value>, _> = serde_json::from_str(json_data);
    assert!(result.is_err());
}

#[should_panic(expected = "invalid type: null, expected a map")]
#[test]
fn test_deserialize_invalid_map() {
    let json_data = "42"; // Non-map input should panic
    let _result: Result<Map<String, Value>, _> = serde_json::from_str(json_data).unwrap();
}

