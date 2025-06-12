// Answer 0

#[test]
fn test_serialize_empty_map() {
    use serde_json::ser::to_string;
    use serde::ser::Serializer;

    let map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    let serialized = to_string(&map).unwrap();
    assert_eq!(serialized, "{}");
}

#[test]
fn test_serialize_single_entry_map() {
    use serde_json::ser::to_string;
    use serde::ser::Serializer;

    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
    
    let serialized = to_string(&map).unwrap();
    assert_eq!(serialized, r#"{"key1":"value1"}"#);
}

#[test]
fn test_serialize_multiple_entries_map() {
    use serde_json::ser::to_string;
    use serde::ser::Serializer;

    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
    map.insert("key2".to_string(), serde_json::Value::String("value2".to_string()));
    
    let serialized = to_string(&map).unwrap();
    assert_eq!(serialized, r#"{"key1":"value1","key2":"value2"}"#);
}

