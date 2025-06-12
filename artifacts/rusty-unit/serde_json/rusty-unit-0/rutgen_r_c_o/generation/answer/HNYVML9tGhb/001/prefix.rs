// Answer 0

#[test]
fn test_is_u64_with_null() {
    let v = Value::Null;
    v.is_u64();
}

#[test]
fn test_is_u64_with_bool_false() {
    let v = Value::Bool(false);
    v.is_u64();
}

#[test]
fn test_is_u64_with_bool_true() {
    let v = Value::Bool(true);
    v.is_u64();
}

#[test]
fn test_is_u64_with_string() {
    let v = Value::String(String::from("test"));
    v.is_u64();
}

#[test]
fn test_is_u64_with_empty_array() {
    let v = Value::Array(Vec::new());
    v.is_u64();
}

#[test]
fn test_is_u64_with_empty_object() {
    let v = Value::Object(Map::new());
    v.is_u64();
}

