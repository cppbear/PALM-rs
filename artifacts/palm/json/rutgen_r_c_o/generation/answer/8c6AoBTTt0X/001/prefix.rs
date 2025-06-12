// Answer 0

#[test]
fn test_value_is_null_with_null() {
    let value = Value::Null;
    value.is_null();
}

#[test]
fn test_value_is_null_with_bool_true() {
    let value = Value::Bool(true);
    value.is_null();
}

#[test]
fn test_value_is_null_with_bool_false() {
    let value = Value::Bool(false);
    value.is_null();
}

#[test]
fn test_value_is_null_with_number() {
    let value = Value::Number(Number { n: 0 });
    value.is_null();
}

#[test]
fn test_value_is_null_with_empty_string() {
    let value = Value::String(String::from(""));
    value.is_null();
}

#[test]
fn test_value_is_null_with_empty_array() {
    let value = Value::Array(Vec::new());
    value.is_null();
}

#[test]
fn test_value_is_null_with_empty_object() {
    let value = Value::Object(Map::new());
    value.is_null();
}

#[test]
fn test_value_is_null_with_array_containing_null() {
    let value = Value::Array(vec![Value::Null]);
    value.is_null();
}

#[test]
fn test_value_is_null_with_object_containing_null() {
    let value = Value::Object(Map::from([(String::from("key"), Value::Null)]));
    value.is_null();
}

#[test]
fn test_value_is_null_with_array_containing_bool_and_null() {
    let value = Value::Array(vec![Value::Bool(true), Value::Null]);
    value.is_null();
}

