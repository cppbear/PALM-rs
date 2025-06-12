// Answer 0

#[test]
fn test_as_i64_null() {
    let value = Value::Null;
    value.as_i64();
}

#[test]
fn test_as_i64_bool_true() {
    let value = Value::Bool(true);
    value.as_i64();
}

#[test]
fn test_as_i64_bool_false() {
    let value = Value::Bool(false);
    value.as_i64();
}

#[test]
fn test_as_i64_string() {
    let value = Value::String(String::from("test"));
    value.as_i64();
}

#[test]
fn test_as_i64_empty_array() {
    let value = Value::Array(Vec::new());
    value.as_i64();
}

#[test]
fn test_as_i64_empty_object() {
    let value = Value::Object(Map::new());
    value.as_i64();
}

