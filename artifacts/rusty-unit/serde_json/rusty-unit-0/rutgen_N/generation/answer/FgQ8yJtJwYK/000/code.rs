// Answer 0

#[test]
fn test_serialize_f64() {
    use serde_json::Value;
    use serde_json::Error;

    struct Serializer;

    impl Serializer {
        fn serialize_f64(self, float: f64) -> Result<Value, Error> {
            Ok(Value::from(float))
        }
    }

    let serializer = Serializer;

    // Test positive float
    let result = serializer.serialize_f64(3.14).unwrap();
    assert_eq!(result, Value::from(3.14));

    // Test negative float
    let result = serializer.serialize_f64(-2.71).unwrap();
    assert_eq!(result, Value::from(-2.71));

    // Test zero
    let result = serializer.serialize_f64(0.0).unwrap();
    assert_eq!(result, Value::from(0.0));
}

