// Answer 0

#[test]
fn test_as_f64_with_null() {
    let value = Value::Null;
    assert_eq!(value.as_f64(), None);
}

#[test]
fn test_as_f64_with_bool() {
    let value = Value::Bool(true);
    assert_eq!(value.as_f64(), None);
}

#[test]
fn test_as_f64_with_string() {
    let value = Value::String(String::from("not a number"));
    assert_eq!(value.as_f64(), None);
}

#[test]
fn test_as_f64_with_array() {
    let value = Value::Array(vec![Value::Bool(false), Value::String(String::from("test"))]);
    assert_eq!(value.as_f64(), None);
}

#[test]
fn test_as_f64_with_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    assert_eq!(value.as_f64(), None);
}

