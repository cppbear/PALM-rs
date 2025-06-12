// Answer 0

#[test]
fn test_is_string_with_string_value() {
    let v = serde_json::json!("Hello, world!");
    assert!(v.is_string());
}

#[test]
fn test_is_string_with_empty_string() {
    let v = serde_json::json!("");
    assert!(v.is_string());
}

#[test]
fn test_is_string_with_boolean_true() {
    let v = serde_json::json!(true);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_boolean_false() {
    let v = serde_json::json!(false);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_number() {
    let v = serde_json::json!(42);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_null() {
    let v = serde_json::json!(null);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_object() {
    let v = serde_json::json!({"key": "value"});
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_array() {
    let v = serde_json::json!([1, 2, 3]);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_string_literal_in_object() {
    let v = serde_json::json!({"string_key": "string_value"});
    assert!(v["string_key"].is_string());
}

#[test]
fn test_is_string_with_string_literal_in_array() {
    let v = serde_json::json!(["string_value", 123]);
    assert!(v[0].is_string());
    assert!(!v[1].is_string());
}

