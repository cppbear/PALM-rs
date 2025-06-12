// Answer 0

#[test]
fn test_eq_f32_with_null_value() {
    let value = Value::Null;
    let other = 3.14;
    assert_eq!(eq_f32(&value, other), false);
}

#[test]
fn test_eq_f32_with_bool_value() {
    let value = Value::Bool(true);
    let other = 3.14;
    assert_eq!(eq_f32(&value, other), false);
}

#[test]
fn test_eq_f32_with_string_value() {
    let value = Value::String("some string".to_string());
    let other = 3.14;
    assert_eq!(eq_f32(&value, other), false);
}

#[test]
fn test_eq_f32_with_array_value() {
    let value = Value::Array(vec![Value::Bool(false)]);
    let other = 3.14;
    assert_eq!(eq_f32(&value, other), false);
}

#[test]
fn test_eq_f32_with_object_value() {
    let value = Value::Object(std::collections::BTreeMap::new());
    let other = 3.14;
    assert_eq!(eq_f32(&value, other), false);
}

