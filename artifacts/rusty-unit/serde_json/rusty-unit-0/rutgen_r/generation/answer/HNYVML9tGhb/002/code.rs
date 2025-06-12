// Answer 0

#[test]
fn test_is_u64_with_valid_integer() {
    use serde_json::Value;
    
    let value = Value::Number(serde_json::Number::from(64));
    assert!(value.is_u64());
}

#[test]
fn test_is_u64_with_negative_integer() {
    use serde_json::Value;
    
    let value = Value::Number(serde_json::Number::from(-64));
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_decimal_number() {
    use serde_json::Value;
    
    let value = Value::Number(serde_json::Number::from_f64(256.0).unwrap());
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_large_integer() {
    use serde_json::Value;
    
    let value = Value::Number(serde_json::Number::from(u64::MAX));
    assert!(value.is_u64());
}

#[test]
fn test_is_u64_with_zero() {
    use serde_json::Value;
    
    let value = Value::Number(serde_json::Number::from(0));
    assert!(value.is_u64());
}

