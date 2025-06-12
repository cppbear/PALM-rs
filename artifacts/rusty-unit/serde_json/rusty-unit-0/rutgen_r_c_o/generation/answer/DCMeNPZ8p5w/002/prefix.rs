// Answer 0

#[test]
fn test_as_array_with_non_empty_array() {
    let value = Value::Array(vec![
        Value::String(String::from("element1")),
        Value::String(String::from("element2")),
    ]);
    let result = value.as_array();
}

#[test]
fn test_as_array_with_empty_array() {
    let value = Value::Array(vec![]);
    let result = value.as_array();
}

#[test]
fn test_as_array_with_nested_array() {
    let value = Value::Array(vec![
        Value::Array(vec![Value::String(String::from("nested1"))]),
        Value::Array(vec![Value::String(String::from("nested2"))]),
    ]);
    let result = value.as_array();
}

#[test]
fn test_as_array_with_mixed_types() {
    let value = Value::Array(vec![
        Value::String(String::from("element")),
        Value::Number(Number { n: 42 }),
        Value::Bool(true),
    ]);
    let result = value.as_array();
}

#[test]
fn test_as_array_with_non_array_value() {
    let value = Value::Bool(true);
    let result = value.as_array();
}

