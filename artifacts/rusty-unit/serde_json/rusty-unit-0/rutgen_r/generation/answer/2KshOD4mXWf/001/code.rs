// Answer 0

#[test]
fn test_serialize_i64_positive_value() {
    let value = 42i64;
    let result = serialize_i64(value);
    assert_eq!(result, Ok(Value::Number(42.into())));
}

#[test]
fn test_serialize_i64_zero_value() {
    let value = 0i64;
    let result = serialize_i64(value);
    assert_eq!(result, Ok(Value::Number(0.into())));
}

#[test]
fn test_serialize_i64_negative_value() {
    let value = -42i64;
    let result = serialize_i64(value);
    assert_eq!(result, Ok(Value::Number((-42).into())));
}

#[test]
fn test_serialize_i64_max_value() {
    let value = i64::MAX;
    let result = serialize_i64(value);
    assert_eq!(result, Ok(Value::Number(i64::MAX.into())));
}

#[test]
fn test_serialize_i64_min_value() {
    let value = i64::MIN;
    let result = serialize_i64(value);
    assert_eq!(result, Ok(Value::Number(i64::MIN.into())));
}

