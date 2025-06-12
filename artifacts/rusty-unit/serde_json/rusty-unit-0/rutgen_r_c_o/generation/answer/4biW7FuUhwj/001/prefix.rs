// Answer 0

#[test]
fn test_is_number_with_null() {
    let value = Value::Null;
    value.is_number();
}

#[test]
fn test_is_number_with_bool() {
    let value = Value::Bool(false);
    value.is_number();
}

#[test]
fn test_is_number_with_string() {
    let value = Value::String(String::from("not a number"));
    value.is_number();
}

#[test]
fn test_is_number_with_empty_array() {
    let value = Value::Array(Vec::new());
    value.is_number();
}

#[test]
fn test_is_number_with_empty_object() {
    let value = Value::Object(Map::new());
    value.is_number();
}

