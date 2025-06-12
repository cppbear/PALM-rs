// Answer 0

#[test]
fn test_is_f64_with_null() {
    let value = Value::Null;
    value.is_f64();
}

#[test]
fn test_is_f64_with_boolean() {
    let value = Value::Bool(true);
    value.is_f64();
}

#[test]
fn test_is_f64_with_string() {
    let value = Value::String("example".to_string());
    value.is_f64();
}

#[test]
fn test_is_f64_with_empty_array() {
    let value = Value::Array(vec![]);
    value.is_f64();
}

#[test]
fn test_is_f64_with_empty_object() {
    let value = Value::Object(Map::new());
    value.is_f64();
}

