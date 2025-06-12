// Answer 0

#[test]
fn test_serialize_i64_positive() {
    let result = serialize_i64(42);
    assert_eq!(result, Ok(Value::Number(42.into())));
}

#[test]
fn test_serialize_i64_negative() {
    let result = serialize_i64(-42);
    assert_eq!(result, Ok(Value::Number((-42).into())));
}

#[test]
fn test_serialize_i64_zero() {
    let result = serialize_i64(0);
    assert_eq!(result, Ok(Value::Number(0.into())));
}

#[test]
fn test_serialize_i64_large_positive() {
    let result = serialize_i64(1_000_000_000);
    assert_eq!(result, Ok(Value::Number(1_000_000_000.into())));
}

#[test]
fn test_serialize_i64_large_negative() {
    let result = serialize_i64(-1_000_000_000);
    assert_eq!(result, Ok(Value::Number((-1_000_000_000).into())));
}

