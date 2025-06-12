// Answer 0

#[test]
fn test_serialize_i64() {
    struct SerdeJson;

    impl SerdeJson {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(value.into()))
        }
    }

    let serializer = SerdeJson;
    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(42.into()));
}

#[test]
fn test_serialize_i64_negative() {
    struct SerdeJson;

    impl SerdeJson {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(value.into()))
        }
    }

    let serializer = SerdeJson;
    let result = serializer.serialize_i64(-10);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number((-10).into()));
}

#[test]
fn test_serialize_i64_zero() {
    struct SerdeJson;

    impl SerdeJson {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::Number(value.into()))
        }
    }

    let serializer = SerdeJson;
    let result = serializer.serialize_i64(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(0.into()));
}

