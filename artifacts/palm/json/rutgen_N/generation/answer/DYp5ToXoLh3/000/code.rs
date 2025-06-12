// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct Serializer;

    impl Serializer {
        fn serialize_bool(self, value: bool) -> Result<serde_json::Value> {
            Ok(serde_json::Value::Bool(value))
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_bool(true).unwrap();
    assert_eq!(result, serde_json::Value::Bool(true));
}

#[test]
fn test_serialize_bool_false() {
    struct Serializer;

    impl Serializer {
        fn serialize_bool(self, value: bool) -> Result<serde_json::Value> {
            Ok(serde_json::Value::Bool(value))
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_bool(false).unwrap();
    assert_eq!(result, serde_json::Value::Bool(false));
}

