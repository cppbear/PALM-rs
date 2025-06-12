// Answer 0

#[test]
fn test_as_null_when_value_is_null() {
    let value = Value::Null;
    assert_eq!(value.as_null(), Some(()));
}

#[test]
fn test_as_null_when_value_is_bool() {
    let value = Value::Bool(false);
    assert_eq!(value.as_null(), None);
}

#[test]
fn test_as_null_when_value_is_number() {
    let value = Value::Number(Number { n: 10.0 });
    assert_eq!(value.as_null(), None);
}

#[test]
fn test_as_null_when_value_is_string() {
    let value = Value::String(String::from("example"));
    assert_eq!(value.as_null(), None);
}

#[test]
fn test_as_null_when_value_is_array() {
    let value = Value::Array(vec![Value::Null]);
    assert_eq!(value.as_null(), None);
}

#[test]
fn test_as_null_when_value_is_object() {
    let mut map = Map { map: Default::default() };
    map.map.insert(String::from("key"), Value::Null);
    let value = Value::Object(map);
    assert_eq!(value.as_null(), None);
}

