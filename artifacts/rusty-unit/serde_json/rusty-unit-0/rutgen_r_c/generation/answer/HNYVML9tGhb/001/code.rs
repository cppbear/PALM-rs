// Answer 0

#[test]
fn test_is_u64_with_null_value() {
    let v = Value::Null;
    assert!(!v.is_u64());
}

#[test]
fn test_is_u64_with_boolean_value_true() {
    let v = Value::Bool(true);
    assert!(!v.is_u64());
}

#[test]
fn test_is_u64_with_boolean_value_false() {
    let v = Value::Bool(false);
    assert!(!v.is_u64());
}

#[test]
fn test_is_u64_with_string_value() {
    let v = Value::String(String::from("not a number"));
    assert!(!v.is_u64());
}

#[test]
fn test_is_u64_with_empty_array() {
    let v = Value::Array(Vec::new());
    assert!(!v.is_u64());
}

#[test]
fn test_is_u64_with_non_empty_array() {
    let v = Value::Array(vec![Value::String(String::from("nested string"))]);
    assert!(!v.is_u64());
}

#[test]
fn test_is_u64_with_object() {
    let v = Value::Object(Map { map: MapImpl::new() });
    assert!(!v.is_u64());
}

