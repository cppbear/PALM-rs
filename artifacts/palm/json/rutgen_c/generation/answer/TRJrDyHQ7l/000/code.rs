// Answer 0

#[test]
fn test_is_string_with_string_value() {
    let value = Value::String(String::from("some string"));
    assert!(value.is_string());
}

#[test]
fn test_is_string_with_integer_value() {
    let value = Value::Number(Number { n: 42 });
    assert!(!value.is_string());
}

#[test]
fn test_is_string_with_boolean_value() {
    let value = Value::Bool(true);
    assert!(!value.is_string());
}

#[test]
fn test_is_string_with_null_value() {
    let value = Value::Null;
    assert!(!value.is_string());
}

#[test]
fn test_is_string_with_array_value() {
    let value = Value::Array(vec![Value::String(String::from("item"))]);
    assert!(!value.is_string());
}

#[test]
fn test_is_string_with_object_value() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(map);
    assert!(!value.is_string());
}

