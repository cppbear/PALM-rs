// Answer 0

#[test]
fn test_is_u64_with_positive_integer() {
    let number = Number::from_u128(64).unwrap(); // Create a valid Number from a u128
    let value = Value::Number(number);
    assert!(value.is_u64());
}

#[test]
fn test_is_u64_with_negative_integer() {
    let number = Number::from_i128(-64).unwrap(); // Create a valid Number from a negative i128
    let value = Value::Number(number);
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_float() {
    let number = Number::from_f64(256.0).unwrap(); // Create a valid Number from a floating point
    let value = Value::Number(number);
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_large_integer() {
    let number = Number::from_u128(u64::MAX as u128).unwrap(); // Create a valid Number with maximum u64 value
    let value = Value::Number(number);
    assert!(value.is_u64());
}

#[test]
fn test_is_u64_with_invalid_number() {
    let number = Number::from_f64(123.456).unwrap(); // Create a valid Number from a floating point
    let value = Value::Number(number);
    assert!(!value.is_u64());
}

