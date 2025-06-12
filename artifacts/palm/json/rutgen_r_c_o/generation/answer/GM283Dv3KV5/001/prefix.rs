// Answer 0

#[test]
fn test_as_f64_with_bool() {
    let value = Value::Bool(true);
    value.as_f64();
}

#[test]
fn test_as_f64_with_null() {
    let value = Value::Null;
    value.as_f64();
}

#[test]
fn test_as_f64_with_string() {
    let value = Value::String(String::from("test"));
    value.as_f64();
}

#[test]
fn test_as_f64_with_empty_array() {
    let value = Value::Array(Vec::new());
    value.as_f64();
}

#[test]
fn test_as_f64_with_object() {
    let value = Value::Object(Map::new());
    value.as_f64();
}

