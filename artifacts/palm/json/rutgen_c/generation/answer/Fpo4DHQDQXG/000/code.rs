// Answer 0

#[test]
fn test_is_array_with_empty_array() {
    let value = Value::Array(Vec::new());
    assert!(value.is_array());
}

#[test]
fn test_is_array_with_non_array_value() {
    let value = Value::Bool(true);
    assert!(!value.is_array());

    let value = Value::Number(Number { n: 0 }); // Assuming a default number
    assert!(!value.is_array());

    let value = Value::String(String::from("test"));
    assert!(!value.is_array());

    let value = Value::Null;
    assert!(!value.is_array());

    let value = Value::Object(Map { map: MapImpl::new() }); // Assuming MapImpl is initialized
    assert!(!value.is_array());
}

#[test]
fn test_is_array_with_array_of_numbers() {
    let value = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    assert!(value.is_array());
}

#[test]
fn test_is_array_with_nested_objects() {
    let value = Value::Array(vec![
        Value::Object(Map { map: MapImpl::new() }),
        Value::Object(Map { map: MapImpl::new() }),
    ]);
    assert!(value.is_array());
}

