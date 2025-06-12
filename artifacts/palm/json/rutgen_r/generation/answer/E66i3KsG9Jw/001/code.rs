// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let value: &[u8] = &[];
    let result = serialize_bytes(value);
    let expected: Value = Value::Array(vec![]);
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_serialize_bytes_single_element() {
    let value: &[u8] = &[42];
    let result = serialize_bytes(value);
    let expected: Value = Value::Array(vec![Value::Number(42.into())]);
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_serialize_bytes_multiple_elements() {
    let value: &[u8] = &[1, 2, 3, 4, 255];
    let result = serialize_bytes(value);
    let expected: Value = Value::Array(vec![
        Value::Number(1.into()),
        Value::Number(2.into()),
        Value::Number(3.into()),
        Value::Number(4.into()),
        Value::Number(255.into()),
    ]);
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_serialize_bytes_boundary_values() {
    let value: &[u8] = &[0, 128, 255];
    let result = serialize_bytes(value);
    let expected: Value = Value::Array(vec![
        Value::Number(0.into()),
        Value::Number(128.into()),
        Value::Number(255.into()),
    ]);
    assert_eq!(result, Ok(expected));
}

