// Answer 0

#[test]
fn test_is_array_empty() {
    let value = Value::Array(Vec::<Value>::new());
    value.is_array();
}

#[test]
fn test_is_array_with_null() {
    let value = Value::Array(Vec::<Value>::from([Value::Null]));
    value.is_array();
}

#[test]
fn test_is_array_with_bool() {
    let value = Value::Array(Vec::<Value>::from([Value::Bool(true)]));
    value.is_array();
}

#[test]
fn test_is_array_with_number() {
    let number_value = Number { n: 1 }; 
    let value = Value::Array(Vec::<Value>::from([Value::Number(number_value)]));
    value.is_array();
}

#[test]
fn test_is_array_with_string() {
    let value = Value::Array(Vec::<Value>::from([Value::String(String::from("test"))]));
    value.is_array();
}

#[test]
fn test_is_array_with_nested_array() {
    let value = Value::Array(Vec::<Value>::from([Value::Array(Vec::new())]));
    value.is_array();
}

#[test]
fn test_is_array_with_bool_false() {
    let value = Value::Bool(false);
    value.is_array();
}

#[test]
fn test_is_array_with_string_not_array() {
    let value = Value::String(String::from("not an array"));
    value.is_array();
}

#[test]
fn test_is_array_with_object() {
    let value = Value::Object(Map::<String, Value>::new());
    value.is_array();
}

#[test]
fn test_is_array_with_null_value() {
    let value = Value::Null;
    value.is_array();
}

