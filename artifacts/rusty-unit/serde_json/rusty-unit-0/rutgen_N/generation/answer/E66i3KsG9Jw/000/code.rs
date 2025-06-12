// Answer 0

#[test]
fn test_serialize_bytes() {
    use serde_json::Value;

    fn serialize_bytes(value: &[u8]) -> Result<Value, serde_json::Error> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }

    // Test with an empty slice
    let result = serialize_bytes(&[]);
    assert_eq!(result.unwrap(), Value::Array(vec![]));

    // Test with a single byte
    let result = serialize_bytes(&[1]);
    assert_eq!(result.unwrap(), Value::Array(vec![Value::Number(1.into())]));

    // Test with multiple bytes
    let result = serialize_bytes(&[1, 2, 3]);
    assert_eq!(
        result.unwrap(),
        Value::Array(vec![
            Value::Number(1.into()),
            Value::Number(2.into()),
            Value::Number(3.into())
        ])
    );
}

#[test]
fn test_serialize_bytes_boundary() {
    use serde_json::Value;

    fn serialize_bytes(value: &[u8]) -> Result<Value, serde_json::Error> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }

    // Test with maximum u8 value
    let result = serialize_bytes(&[255]);
    assert_eq!(result.unwrap(), Value::Array(vec![Value::Number(255.into())]));

    // Test with a slice containing all u8 values
    let result = serialize_bytes(&(0u8..=255).collect::<Vec<u8>>());
    let mut expected = Vec::new();
    for i in 0..=255 {
        expected.push(Value::Number(i.into()));
    }
    assert_eq!(result.unwrap(), Value::Array(expected));
}

