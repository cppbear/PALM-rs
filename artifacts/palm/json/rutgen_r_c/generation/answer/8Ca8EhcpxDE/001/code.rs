// Answer 0

#[test]
fn test_serialize_struct_variant_end() {
    struct TestSerializeStructVariant {
        name: String,
        map: Map<String, Value>,
    }

    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Bool(true));

    let variant = TestSerializeStructVariant {
        name: "test_variant".to_string(),
        map,
    };

    let result = variant.end();
    let expected_value = Value::Object(Map::new().insert("test_variant".to_string(), Value::Object(variant.map)));

    match result {
        Ok(Value::Object(obj)) => {
            assert!(obj.contains_key("test_variant"));
            if let Some(Value::Object(inner_map)) = obj.get("test_variant") {
                assert_eq!(inner_map.len(), 2);
                assert!(inner_map.contains_key("key1"));
                assert!(inner_map.contains_key("key2"));
            } else {
                panic!("Expected inner Value::Object");
            }
        },
        _ => {
            panic!("Expected Ok(Value::Object(...))");
        }
    }
}

#[test]
fn test_serialize_struct_variant_empty_map() {
    struct TestSerializeStructVariant {
        name: String,
        map: Map<String, Value>,
    }

    let variant = TestSerializeStructVariant {
        name: "empty_variant".to_string(),
        map: Map::new(),
    };

    let result = variant.end();
    let expected_value = Value::Object(Map::new().insert("empty_variant".to_string(), Value::Object(Map::new())));

    match result {
        Ok(Value::Object(obj)) => {
            assert!(obj.contains_key("empty_variant"));
            if let Some(Value::Object(inner_map)) = obj.get("empty_variant") {
                assert!(inner_map.is_empty());
            } else {
                panic!("Expected inner Value::Object");
            }
        },
        _ => {
            panic!("Expected Ok(Value::Object(...))");
        }
    }
}

