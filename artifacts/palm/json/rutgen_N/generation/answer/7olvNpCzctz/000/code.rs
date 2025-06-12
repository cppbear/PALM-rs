// Answer 0

#[test]
fn test_serialize_f32() {
    struct Serializer;

    impl Serializer {
        fn serialize_f32(self, float: f32) -> Result<serde_json::Value, serde_json::Error> {
            Ok(serde_json::Value::from(float))
        }
    }

    let serializer = Serializer;

    // Test serialization of a regular float
    let result = serializer.serialize_f32(3.14);
    assert_eq!(result.unwrap(), serde_json::Value::from(3.14));

    // Test serialization of a negative float
    let result = serializer.serialize_f32(-2.5);
    assert_eq!(result.unwrap(), serde_json::Value::from(-2.5));

    // Test serialization of zero
    let result = serializer.serialize_f32(0.0);
    assert_eq!(result.unwrap(), serde_json::Value::from(0.0));
}

