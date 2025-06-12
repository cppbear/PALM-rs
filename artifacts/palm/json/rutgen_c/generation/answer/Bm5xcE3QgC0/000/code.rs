// Answer 0

#[test]
fn test_eq_bool_true() {
    let value = Value::Bool(true);
    assert!(eq_bool(&value, true));
}

#[test]
fn test_eq_bool_false() {
    let value = Value::Bool(false);
    assert!(eq_bool(&value, false));
}

#[test]
fn test_eq_bool_null() {
    let value = Value::Null;
    assert!(!eq_bool(&value, true));
    assert!(!eq_bool(&value, false));
}

#[test]
fn test_eq_bool_string() {
    let value = Value::String(String::from("a string"));
    assert!(!eq_bool(&value, true));
    assert!(!eq_bool(&value, false));
}

#[test]
fn test_eq_bool_number() {
    let value = Value::Number(Number::from(42)); // assuming Number can be created this way
    assert!(!eq_bool(&value, true));
    assert!(!eq_bool(&value, false));
}

#[test]
fn test_eq_bool_array() {
    let value = Value::Array(vec![Value::Bool(true)]);
    assert!(!eq_bool(&value, true));
    assert!(!eq_bool(&value, false));
}

#[test]
fn test_eq_bool_object() {
    let value = Value::Object(Map::new()); // assuming a new empty Map can be created this way
    assert!(!eq_bool(&value, true));
    assert!(!eq_bool(&value, false));
}

