// Answer 0

#[test]
fn test_is_number_with_number_value() {
    let value = Value::Number(Number { n: 42 });
    assert!(value.is_number());
}

#[test]
fn test_is_number_with_null_value() {
    let value = Value::Null;
    assert!(!value.is_number());
}

#[test]
fn test_is_number_with_bool_value() {
    let value = Value::Bool(true);
    assert!(!value.is_number());
}

#[test]
fn test_is_number_with_string_value() {
    let value = Value::String(String::from("example"));
    assert!(!value.is_number());
}

#[test]
fn test_is_number_with_array_value() {
    let value = Value::Array(vec![Value::Number(Number { n: 3 }), Value::Number(Number { n: 4 })]);
    assert!(!value.is_number());
}

#[test]
fn test_is_number_with_object_value() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::Number(Number { n: 5 }));
    let value = Value::Object(map);
    assert!(!value.is_number());
}

