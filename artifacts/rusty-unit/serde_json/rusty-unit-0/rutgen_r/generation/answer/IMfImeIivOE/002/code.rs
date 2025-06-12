// Answer 0

#[test]
fn test_serialize_field_success() {
    use serde_json::json;
    use serde::Serialize;
    use std::collections::HashMap;
    use serde_json::Result;

    struct MockSerializer {
        map: HashMap<String, serde_json::Value>,
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                map: HashMap::new(),
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

    let mut serializer = MockSerializer::new();
    let key = "test_key";
    let value = &json!("test_value");

    let result = serializer.serialize_field(key, value);
    assert!(result.is_ok());
    assert_eq!(serializer.map.get(key), Some(&json!("test_value")));
}

#[test]
#[should_panic]
fn test_serialize_field_fail_due_to_non_serializable() {
    use std::collections::HashMap;
    use serde_json::Result;

    struct MockSerializer {
        map: HashMap<String, serde_json::Value>,
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                map: HashMap::new(),
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

    let mut serializer = MockSerializer::new();
    let key = "test_key";

    // Attempting to serialize a non-serializable structure
    let value = &f64::NAN; // f64::NAN is not serializable

    let _result = serializer.serialize_field(key, value);
}

