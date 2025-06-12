// Answer 0

#[test]
fn test_is_i64_with_integer_value() {
    let number = Number::from_i128(64).unwrap();
    let value = Value::Number(number);
    assert!(value.is_i64());
}

#[test]
fn test_is_i64_with_large_unsigned_value() {
    let number = Number::from_u128(u128::MAX).unwrap(); // Assuming MAX value is out of range for i64.
    let value = Value::Number(number);
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_floating_point_value() {
    let number = Number::from_f64(256.0).unwrap();
    let value = Value::Number(number);
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_null_value() {
    let value = Value::Null;
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_boolean_value() {
    let value = Value::Bool(true);
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_string_value() {
    let value = Value::String(String::from("string"));
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_empty_array() {
    let value = Value::Array(Vec::new());
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_object() {
    let value = Value::Object(Map { map: Default::default() });
    assert!(!value.is_i64());
}

