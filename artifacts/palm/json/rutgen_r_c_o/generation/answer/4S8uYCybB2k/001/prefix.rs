// Answer 0

#[test]
fn test_as_u64_with_null() {
    let v = Value::Null;
    let result = v.as_u64();
}

#[test]
fn test_as_u64_with_boolean() {
    let v = Value::Bool(true);
    let result = v.as_u64();
}

#[test]
fn test_as_u64_with_string() {
    let v = Value::String(String::from("a string"));
    let result = v.as_u64();
}

#[test]
fn test_as_u64_with_array() {
    let v = Value::Array(Vec::new());
    let result = v.as_u64();
}

#[test]
fn test_as_u64_with_object() {
    let v = Value::Object(Map::new());
    let result = v.as_u64();
}

