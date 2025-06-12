// Answer 0

#[test]
fn test_as_i64_with_valid_i64() {
    let number = Number::from_i128(64).unwrap();
    let value = Value::Number(number);
    assert_eq!(value.as_i64(), Some(64));
}

#[test]
fn test_as_i64_with_large_u64() {
    let number = Number::from_u128(u128::MAX).unwrap(); // value exceeds i64
    let value = Value::Number(number);
    assert_eq!(value.as_i64(), None);
}

#[test]
fn test_as_i64_with_float() {
    let number = Number::from_f64(256.0).unwrap(); // floating-point value
    let value = Value::Number(number);
    assert_eq!(value.as_i64(), None);
}

#[test]
fn test_as_i64_with_non_number_value() {
    let value = Value::Bool(true); // a non-number
    assert_eq!(value.as_i64(), None);
}

#[test]
fn test_as_i64_with_negative_number() {
    let number = Number::from_i128(-10).unwrap(); // negative value as i128
    let value = Value::Number(number);
    assert_eq!(value.as_i64(), Some(-10));
}

