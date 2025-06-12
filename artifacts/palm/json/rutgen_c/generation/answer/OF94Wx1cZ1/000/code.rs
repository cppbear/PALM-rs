// Answer 0

#[test]
fn test_is_boolean_true() {
    let value = Value::Bool(true);
    assert!(value.is_boolean());
}

#[test]
fn test_is_boolean_false() {
    let value_string = Value::String(String::from("false"));
    assert!(!value_string.is_boolean());

    let value_null = Value::Null;
    assert!(!value_null.is_boolean());

    let value_array = Value::Array(vec![Value::Bool(true)]);
    assert!(!value_array.is_boolean());

    let value_object = Value::Object(Map { map: Default::default() });
    assert!(!value_object.is_boolean());
}

