// Answer 0

#[test]
fn test_serialize_map_end_with_map() {
    struct SerializeMap {
        map: std::collections::HashMap<String, serde_json::Value>,
    }

    impl SerializeMap {
        fn end(self) -> Result<serde_json::Value> {
            match self {
                SerializeMap { map } => Ok(serde_json::Value::Object(map)),
            }
        }
    }

    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
    map.insert("key2".to_string(), serde_json::Value::Number(serde_json::Number::from(42)));
  
    let serialize_map = SerializeMap { map };
    let result = serialize_map.end();

    assert!(result.is_ok());
    if let Ok(value) = result {
        assert!(value.is_object());
        assert_eq!(value.as_object().unwrap().len(), 2);
        assert_eq!(value["key1"], "value1");
        assert_eq!(value["key2"], 42);
    }
}

