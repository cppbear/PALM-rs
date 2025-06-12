// Answer 0

#[test]
fn test_as_array_with_non_empty_array() {
    let value = Value::Array(vec![
        Value::String(String::from("first")),
        Value::String(String::from("second")),
    ]);
    assert_eq!(value.as_array().unwrap().len(), 2);
}

#[test]
fn test_as_array_with_empty_array() {
    let value = Value::Array(vec![]);
    assert_eq!(value.as_array().unwrap().len(), 0);
}

#[test]
fn test_as_array_with_non_array_value() {
    let value = Value::Bool(true);
    assert_eq!(value.as_array(), None);
}

#[test]
fn test_as_array_with_nested_array() {
    let value = Value::Array(vec![
        Value::Array(vec![Value::String(String::from("nested"))]),
    ]);
    if let Some(inner_array) = value.as_array() {
        assert_eq!(inner_array.len(), 1);
        assert_eq!(inner_array[0].as_array().unwrap().len(), 1);
    } else {
        panic!("Expected value to be an array");
    }
}

