// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let value: &[u8] = &[];
    let result = serialize_bytes(value);
    assert_eq!(result, Ok(Value::Array(vec![])));
}

#[test]
fn test_serialize_bytes_single_zero() {
    let value: &[u8] = &[0];
    let result = serialize_bytes(value);
    assert_eq!(result, Ok(Value::Array(vec![Value::Number(0.into())])));
}

#[test]
fn test_serialize_bytes_single_max() {
    let value: &[u8] = &[255];
    let result = serialize_bytes(value);
    assert_eq!(result, Ok(Value::Array(vec![Value::Number(255.into())])));
}

#[test]
fn test_serialize_bytes_multiple() {
    let value: &[u8] = &[1, 2, 3, 4, 5];
    let result = serialize_bytes(value);
    let expected = vec![
        Value::Number(1.into()),
        Value::Number(2.into()),
        Value::Number(3.into()),
        Value::Number(4.into()),
        Value::Number(5.into()),
    ];
    assert_eq!(result, Ok(Value::Array(expected)));
}

#[test]
fn test_serialize_bytes_large() {
    let value: &[u8] = &[100, 200, 150, 250];
    let result = serialize_bytes(value);
    let expected = vec![
        Value::Number(100.into()),
        Value::Number(200.into()),
        Value::Number(150.into()),
        Value::Number(250.into()),
    ];
    assert_eq!(result, Ok(Value::Array(expected)));
}

