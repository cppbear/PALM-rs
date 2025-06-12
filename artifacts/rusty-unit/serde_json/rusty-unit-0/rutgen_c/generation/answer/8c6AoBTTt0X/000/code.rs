// Answer 0

#[test]
fn test_is_null_on_null_value() {
    let value = Value::Null;
    assert!(value.is_null());
}

#[test]
fn test_is_null_on_non_null_value() {
    let value = Value::Bool(false);
    assert!(!value.is_null());
}

#[test]
fn test_is_null_on_number_value() {
    let value = Value::Number(Number { n: 0 });
    assert!(!value.is_null());
}

#[test]
fn test_is_null_on_string_value() {
    let value = Value::String(String::from("test"));
    assert!(!value.is_null());
}

#[test]
fn test_is_null_on_array_value() {
    let value = Value::Array(vec![Value::Null]);
    assert!(!value.is_null());
}

#[test]
fn test_is_null_on_object_value() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::Bool(true));
    let value = Value::Object(map);
    assert!(!value.is_null());
}

