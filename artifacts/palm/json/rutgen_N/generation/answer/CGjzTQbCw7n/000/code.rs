// Answer 0

#[test]
fn test_serialize_str() {
    struct Serializer;

    impl Serializer {
        fn serialize_str(self, value: &str) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::String(value.to_owned()))
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_str("test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::String("test".to_owned()));
}

#[test]
fn test_serialize_empty_str() {
    struct Serializer;

    impl Serializer {
        fn serialize_str(self, value: &str) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::String(value.to_owned()))
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_str("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::String("".to_owned()));
}

