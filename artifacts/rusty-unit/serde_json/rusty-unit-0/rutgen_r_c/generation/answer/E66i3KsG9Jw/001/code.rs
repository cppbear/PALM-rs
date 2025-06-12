// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_bytes(&[]);
    assert_eq!(result, Ok(Value::Array(Vec::new())));
}

#[test]
fn test_serialize_bytes_single_byte() {
    let serializer = Serializer;
    let result = serializer.serialize_bytes(&[42]);
    let expected = Value::Array(vec![Value::Number(42.into())]);
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_serialize_bytes_multiple_bytes() {
    let serializer = Serializer;
    let input = &[0, 1, 2, 3, 255];
    let result = serializer.serialize_bytes(input);
    let expected = Value::Array(vec![
        Value::Number(0.into()),
        Value::Number(1.into()),
        Value::Number(2.into()),
        Value::Number(3.into()),
        Value::Number(255.into()),
    ]);
    assert_eq!(result, Ok(expected));
}

