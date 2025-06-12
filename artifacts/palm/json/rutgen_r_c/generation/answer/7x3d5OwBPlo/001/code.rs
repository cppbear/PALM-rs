// Answer 0

#[test]
fn test_as_number_with_null() {
    let v = serde_json::Value::Null;
    assert_eq!(v.as_number(), None);
}

#[test]
fn test_as_number_with_boolean() {
    let v = serde_json::Value::Bool(true);
    assert_eq!(v.as_number(), None);
}

#[test]
fn test_as_number_with_string() {
    let v = serde_json::Value::String("test".to_string());
    assert_eq!(v.as_number(), None);
}

#[test]
fn test_as_number_with_array() {
    let v = serde_json::Value::Array(vec![serde_json::Value::Number(serde_json::Number::from(1))]);
    assert_eq!(v.as_number(), None);
}

#[test]
fn test_as_number_with_object() {
    let v = serde_json::Value::Object(serde_json::Map::new());
    assert_eq!(v.as_number(), None);
}

