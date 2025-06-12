// Answer 0

#[test]
fn test_is_u64_with_positive_integer() {
    let value = Value::Number(Number::from_u128(64).unwrap());
    assert!(value.is_u64());
}

#[test]
fn test_is_u64_with_negative_integer() {
    let value = Value::Number(Number::from_i128(-64).unwrap());
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_decimal_number() {
    let value = Value::Number(Number::from_f64(256.0).unwrap());
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_null_value() {
    let value = Value::Null;
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_boolean_value() {
    let value = Value::Bool(true);
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_string_value() {
    let value = Value::String(String::from("not a number"));
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_array_value() {
    let value = Value::Array(vec![]);
    assert!(!value.is_u64());
}

#[test]
fn test_is_u64_with_object_value() {
    let value = Value::Object(Map::new());
    assert!(!value.is_u64());
}

