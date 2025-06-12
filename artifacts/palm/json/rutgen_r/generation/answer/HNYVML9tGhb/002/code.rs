// Answer 0

#[test]
fn test_is_u64_valid_integer() {
    use serde_json::Value;

    let valid_u64 = Value::Number(serde_json::Number::from(64));
    assert!(valid_u64.is_u64());
}

#[test]
fn test_is_u64_negative_integer() {
    use serde_json::Value;

    let negative_integer = Value::Number(serde_json::Number::from(-64));
    assert!(!negative_integer.is_u64());
}

#[test]
fn test_is_u64_decimal_number() {
    use serde_json::Value;

    let decimal_number = Value::Number(serde_json::Number::from_f64(256.0).unwrap());
    assert!(!decimal_number.is_u64());
}

#[test]
fn test_is_u64_zero() {
    use serde_json::Value;

    let zero = Value::Number(serde_json::Number::from(0));
    assert!(zero.is_u64());
}

#[test]
fn test_is_u64_large_integer() {
    use serde_json::Value;

    let large_integer = Value::Number(serde_json::Number::from(u64::MAX));
    assert!(large_integer.is_u64());
}

#[test]
fn test_is_u64_overflow() {
    use serde_json::Value;

    let overflow_integer = Value::Number(serde_json::Number::from(u64::MAX as i64 + 1));
    assert!(!overflow_integer.is_u64());
}

