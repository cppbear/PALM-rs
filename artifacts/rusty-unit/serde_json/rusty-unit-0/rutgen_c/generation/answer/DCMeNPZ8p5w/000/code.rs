// Answer 0

#[test]
fn test_as_array_with_empty_array() {
    let value = Value::Array(Vec::new());
    assert_eq!(value.as_array().unwrap().len(), 0);
}

#[test]
fn test_as_array_with_non_array() {
    let value = Value::Bool(true);
    assert_eq!(value.as_array(), None);
}

#[test]
fn test_as_array_with_single_element() {
    let value = Value::Array(vec![Value::String(String::from("single"))]);
    assert_eq!(value.as_array().unwrap().len(), 1);
}

#[test]
fn test_as_array_with_multiple_elements() {
    let value = Value::Array(vec![
        Value::String(String::from("element1")),
        Value::String(String::from("element2")),
    ]);
    assert_eq!(value.as_array().unwrap().len(), 2);
}

#[test]
fn test_as_array_with_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    assert_eq!(value.as_array(), None);
}

