// Answer 0

#[test]
fn test_as_object_mut_with_null() {
    let mut value = Value::Null;
    value.as_object_mut();
}

#[test]
fn test_as_object_mut_with_bool_false() {
    let mut value = Value::Bool(false);
    value.as_object_mut();
}

#[test]
fn test_as_object_mut_with_number() {
    let mut value = Value::Number(Number { n: 0 });
    value.as_object_mut();
}

#[test]
fn test_as_object_mut_with_empty_string() {
    let mut value = Value::String(String::from(""));
    value.as_object_mut();
}

#[test]
fn test_as_object_mut_with_empty_array() {
    let mut value = Value::Array(Vec::new());
    value.as_object_mut();
}

