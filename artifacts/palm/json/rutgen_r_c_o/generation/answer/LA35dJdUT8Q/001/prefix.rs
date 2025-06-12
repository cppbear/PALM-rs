// Answer 0

#[test]
fn test_as_object_with_bool_true() {
    let value = Value::Bool(true);
    value.as_object();
}

#[test]
fn test_as_object_with_bool_false() {
    let value = Value::Bool(false);
    value.as_object();
}

#[test]
fn test_as_object_with_null() {
    let value = Value::Null;
    value.as_object();
}

#[test]
fn test_as_object_with_string() {
    let value = Value::String(String::from("test"));
    value.as_object();
}

#[test]
fn test_as_object_with_empty_array() {
    let value = Value::Array(Vec::new());
    value.as_object();
}

#[test]
fn test_as_object_with_number() {
    let value = Value::Number(Number { n: 0 });
    value.as_object();
}

