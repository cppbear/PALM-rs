// Answer 0

#[test]
fn test_is_i64_with_null() {
    let value = serde_json::Value::Null;
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_boolean_true() {
    let value = serde_json::Value::Bool(true);
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_boolean_false() {
    let value = serde_json::Value::Bool(false);
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_string() {
    let value = serde_json::Value::String(String::from("not a number"));
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_float() {
    let value = serde_json::Value::Number(serde_json::Number::from_f64(12.34).unwrap());
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_array() {
    let value = serde_json::Value::Array(vec![
        serde_json::Value::Number(serde_json::Number::from(1).unwrap()),
        serde_json::Value::Number(serde_json::Number::from(2).unwrap()),
    ]);
    assert!(!value.is_i64());
}

#[test]
fn test_is_i64_with_object() {
    let value = serde_json::Value::Object(serde_json::Map::new());
    assert!(!value.is_i64());
}

