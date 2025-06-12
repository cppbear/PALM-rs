// Answer 0

#[test]
fn test_is_array_true() {
    use serde_json::json;
    let value = json!([1, 2, 3]);
    assert!(value.is_array());
}

#[test]
fn test_is_array_false() {
    use serde_json::json;
    let value = json!({"key": "value"});
    assert!(!value.is_array());
}

#[test]
fn test_is_array_empty_array() {
    use serde_json::json;
    let value = json!([]);
    assert!(value.is_array());
}

#[test]
fn test_is_array_null() {
    use serde_json::json;
    let value = json!(null);
    assert!(!value.is_array());
}

#[test]
fn test_is_array_boolean() {
    use serde_json::json;
    let value = json!(true);
    assert!(!value.is_array());
}

