// Answer 0

#[test]
fn test_serialize_field_insert_string() {
    use serde_json::value::Value;
    use serde::ser::Serialize;

    struct TestSerializer {
        map: std::collections::HashMap<String, Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                map: std::collections::HashMap::new(),
            }
        }

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.map.insert(String::from(key), serde_json::to_value(value)?);
            Ok(())
        }
    }

    let mut serializer = TestSerializer::new();
    let result = serializer.serialize_field("key", &"value");
    assert!(result.is_ok());
    assert_eq!(serializer.map.get("key").unwrap(), &serde_json::json!("value"));
}

#[test]
fn test_serialize_field_insert_integer() {
    use serde_json::value::Value;
    use serde::ser::Serialize;

    struct TestSerializer {
        map: std::collections::HashMap<String, Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                map: std::collections::HashMap::new(),
            }
        }

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.map.insert(String::from(key), serde_json::to_value(value)?);
            Ok(())
        }
    }

    let mut serializer = TestSerializer::new();
    let result = serializer.serialize_field("number", &42);
    assert!(result.is_ok());
    assert_eq!(serializer.map.get("number").unwrap(), &serde_json::json!(42));
}

#[test]
fn test_serialize_field_insert_nested_struct() {
    use serde_json::value::Value;
    use serde::ser::Serialize;

    #[derive(Serialize)]
    struct Nested {
        field: String,
    }

    struct TestSerializer {
        map: std::collections::HashMap<String, Value>,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                map: std::collections::HashMap::new(),
            }
        }

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.map.insert(String::from(key), serde_json::to_value(value)?);
            Ok(())
        }
    }

    let mut serializer = TestSerializer::new();
    let result = serializer.serialize_field("nested", &Nested { field: "data".to_string() });
    assert!(result.is_ok());
    assert_eq!(serializer.map.get("nested").unwrap(), &serde_json::json!({"field": "data"}));
}

