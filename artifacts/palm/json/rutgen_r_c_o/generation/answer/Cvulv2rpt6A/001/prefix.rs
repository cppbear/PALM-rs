// Answer 0

#[test]
fn test_eq_f32_with_null_value() {
    let value = Value::Null;
    let other = 1.0f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_bool_value() {
    let value = Value::Bool(false);
    let other = 1.0f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_string_value() {
    let value = Value::String(String::from("test"));
    let other = 1.0f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_empty_array() {
    let value = Value::Array(vec![]);
    let other = 1.0f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_empty_object() {
    let value = Value::Object(std::collections::BTreeMap::new());
    let other = 1.0f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_min_value() {
    let value = Value::Null;
    let other = f32::MIN;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_max_value() {
    let value = Value::Bool(true);
    let other = f32::MAX;
    eq_f32(&value, other);
}

