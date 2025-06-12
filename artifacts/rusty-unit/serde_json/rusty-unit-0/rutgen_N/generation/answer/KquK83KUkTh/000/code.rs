// Answer 0

#[test]
fn test_serialize_empty_map() {
    use serde_json::Value;
    use serde::ser::{Serializer, SerializeMap};

    struct TestMap {
        data: std::collections::HashMap<String, Value>,
    }

    impl std::ops::Deref for TestMap {
        type Target = std::collections::HashMap<String, Value>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new(),
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let test_map = TestMap::new();

    let serialized = serde_json::to_string(&test_map).unwrap();
    assert_eq!(serialized, "{}");
}

#[test]
fn test_serialize_non_empty_map() {
    use serde_json::Value;
    use serde::ser::{Serializer, SerializeMap};

    struct TestMap {
        data: std::collections::HashMap<String, Value>,
    }

    impl std::ops::Deref for TestMap {
        type Target = std::collections::HashMap<String, Value>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl TestMap {
        fn new(data: std::collections::HashMap<String, Value>) -> Self {
            Self { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut data = std::collections::HashMap::new();
    data.insert("key1".to_string(), Value::String("value1".to_string()));
    data.insert("key2".to_string(), Value::String("value2".to_string()));
    
    let test_map = TestMap::new(data);

    let serialized = serde_json::to_string(&test_map).unwrap();
    assert_eq!(serialized, r#"{"key1":"value1","key2":"value2"}"#);
}

