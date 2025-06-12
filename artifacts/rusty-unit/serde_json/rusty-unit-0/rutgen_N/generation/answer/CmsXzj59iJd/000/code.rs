// Answer 0

#[derive(Default)]
struct Serializer;

impl Serializer {
    fn serialize_i64(self, value: i64) -> Result<Value> {
        // Dummy implementation for the purpose of testing
        Ok(Value::Number(serde_json::Number::from(value)))
    }

    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }
}

#[test]
fn test_serialize_i32() {
    let serializer = Serializer::default();
    let result = serializer.serialize_i32(42).unwrap();
    assert_eq!(result, Value::Number(serde_json::Number::from(42)));
}

#[test]
fn test_serialize_i32_negative() {
    let serializer = Serializer::default();
    let result = serializer.serialize_i32(-42).unwrap();
    assert_eq!(result, Value::Number(serde_json::Number::from(-42)));
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = Serializer::default();
    let result = serializer.serialize_i32(0).unwrap();
    assert_eq!(result, Value::Number(serde_json::Number::from(0)));
}

