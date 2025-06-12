// Answer 0

#[test]
fn test_serialize_i8_success() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value> {
            Ok(serde_json::json!(value))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(42);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::json!(42));
}

#[test]
fn test_serialize_i8_negative_value() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value> {
            Ok(serde_json::json!(value))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(-10);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::json!(-10));
}

#[test]
fn test_serialize_i8_zero() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_i64(self, value: i64) -> Result<serde_json::Value> {
            Ok(serde_json::json!(value))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(0);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::json!(0));
}

