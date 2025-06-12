// Answer 0

#[test]
fn test_is_object_for_empty_object() {
    let obj = serde_json::json!({});
    assert!(obj.is_object());
}

#[test]
fn test_is_object_for_nested_object() {
    let obj = serde_json::json!({ "a": { "nested": true } });
    assert!(obj.is_object());
    assert!(obj["a"].is_object());
}

#[test]
fn test_is_object_for_array() {
    let obj = serde_json::json!([1, 2, 3]);
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_for_mixed_value() {
    let obj = serde_json::json!({ "a": 1, "b": [1, 2, 3], "c": { "nested": false } });
    assert!(obj.is_object());
    assert!(!obj["b"].is_object());
    assert!(obj["c"].is_object());
}

#[test]
fn test_is_object_for_string_value() {
    let obj = serde_json::json!("a string");
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_for_boolean_value() {
    let obj = serde_json::json!(true);
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_for_null_value() {
    let obj = serde_json::json!(null);
    assert!(!obj.is_object());
}

