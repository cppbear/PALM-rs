// Answer 0

#[test]
fn test_is_object_with_valid_object() {
    let obj = serde_json::json!({ "key": "value" });
    assert!(obj.is_object());
}

#[test]
fn test_is_object_with_nested_object() {
    let obj = serde_json::json!({ "outer": { "inner": "value" } });
    assert!(obj.is_object());
    assert!(obj["outer"].is_object());
}

#[test]
fn test_is_object_with_array() {
    let obj = serde_json::json!([1, 2, 3]);
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_with_empty_object() {
    let obj = serde_json::json!({});
    assert!(obj.is_object());
}

#[test]
fn test_is_object_with_string() {
    let obj = serde_json::json!("string_value");
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_with_boolean() {
    let obj = serde_json::json!(true);
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_with_null() {
    let obj = serde_json::json!(null);
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_with_mixed_content() {
    let obj = serde_json::json!({ "array": [1, 2], "bool": false });
    assert!(obj.is_object());
    assert!(!obj["array"].is_object());
    assert!(!obj["bool"].is_object());
}

#[test]
fn test_is_object_with_special_characters() {
    let obj = serde_json::json!({ "special!@#$": "value" });
    assert!(obj.is_object());
}

