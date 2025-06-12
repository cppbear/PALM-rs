// Answer 0

#[test]
fn test_as_number_null() {
    let value = Value::Null;
    value.as_number();
}

#[test]
fn test_as_number_bool_true() {
    let value = Value::Bool(true);
    value.as_number();
}

#[test]
fn test_as_number_bool_false() {
    let value = Value::Bool(false);
    value.as_number();
}

#[test]
fn test_as_number_string() {
    let value = Value::String(String::from("test"));
    value.as_number();
}

#[test]
fn test_as_number_empty_array() {
    let value = Value::Array(Vec::new());
    value.as_number();
}

#[test]
fn test_as_number_object() {
    let value = Value::Object(Map::new());
    value.as_number();
}

