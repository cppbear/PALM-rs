// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_bytes(&[]).unwrap();
    assert_eq!(result, Value::Array(Vec::new()));
}

#[test]
fn test_serialize_bytes_single_value() {
    let serializer = Serializer;
    let result = serializer.serialize_bytes(&[42]).unwrap();
    assert_eq!(result, Value::Array(vec![Value::Number(42.into())]));
}

#[test]
fn test_serialize_bytes_multiple_values() {
    let serializer = Serializer;
    let input = &[1, 2, 3, 255];
    let expected = Value::Array(vec![
        Value::Number(1.into()),
        Value::Number(2.into()),
        Value::Number(3.into()),
        Value::Number(255.into()),
    ]);
    let result = serializer.serialize_bytes(input).unwrap();
    assert_eq!(result, expected);
}

