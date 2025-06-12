// Answer 0

#[test]
fn test_is_f64_with_f64_value() {
    use serde_json::Value;

    let value = Value::Number(serde_json::Number::from_f64(256.0).unwrap());
    
    assert!(value.is_f64());
}

#[test]
fn test_is_f64_with_i64_value() {
    use serde_json::Value;

    let value = Value::Number(serde_json::Number::from(64)); // i64

    assert!(!value.is_f64());
}

#[test]
fn test_is_f64_with_u64_value() {
    use serde_json::Value;

    let value = Value::Number(serde_json::Number::from(64u64)); // u64

    assert!(!value.is_f64());
}

#[test]
fn test_is_f64_with_negative_i64_value() {
    use serde_json::Value;

    let value = Value::Number(serde_json::Number::from(-64)); // i64

    assert!(!value.is_f64());
}

#[test]
fn test_is_f64_with_null_value() {
    use serde_json::Value;

    let value = Value::Null; // Not a Number

    assert!(!value.is_f64());
}

#[test]
fn test_is_f64_with_boolean_value() {
    use serde_json::Value;

    let value = Value::Bool(true); // Not a Number

    assert!(!value.is_f64());
}

#[test]
fn test_is_f64_with_string_value() {
    use serde_json::Value;

    let value = Value::String("256.0".to_string()); // Not a Number

    assert!(!value.is_f64());
}

