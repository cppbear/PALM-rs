// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_u64(&self, value: u64) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        Ok(serde_json::Value::Number(serde_json::Number::from(value)))
    }

    fn serialize_u32(self, value: u32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.serialize_u64(value as u64)
    }
}

#[test]
fn test_serialize_u32() {
    let serializer = Serializer;

    // Test minimum value for u32
    let result = serializer.serialize_u32(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(0)));

    // Test middle value for u32
    let result = serializer.serialize_u32(100);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(100)));

    // Test maximum value for u32
    let result = serializer.serialize_u32(u32::MAX);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::Value::Number(serde_json::Number::from(u32::MAX as u64)));
}

