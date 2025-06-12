// Answer 0

#[test]
fn test_value_as_u64_with_integer() {
    let value = Value::Number(Number::from_u128(64).unwrap());
    assert_eq!(value.as_u64(), Some(64));
}

#[test]
fn test_value_as_u64_with_negative_integer() {
    let value = Value::Number(Number::from_i128(-64).unwrap());
    assert_eq!(value.as_u64(), None);
}

#[test]
fn test_value_as_u64_with_float() {
    let value = Value::Number(Number::from_f64(256.0).unwrap());
    assert_eq!(value.as_u64(), None);
}

#[test]
fn test_value_as_u64_with_null() {
    let value = Value::Null;
    assert_eq!(value.as_u64(), None);
}

#[test]
fn test_value_as_u64_with_boolean() {
    let value = Value::Bool(true);
    assert_eq!(value.as_u64(), None);
}

#[test]
fn test_value_as_u64_with_string() {
    let value = Value::String(String::from("a string"));
    assert_eq!(value.as_u64(), None);
}

#[test]
fn test_value_as_u64_with_array() {
    let value = Value::Array(vec![
        Value::Number(Number::from_u128(1).unwrap()),
        Value::Number(Number::from_u128(2).unwrap()),
    ]);
    assert_eq!(value.as_u64(), None);
}

#[test]
fn test_value_as_u64_with_object() {
    let value = Value::Object(Map::new());
    assert_eq!(value.as_u64(), None);
}

