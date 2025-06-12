// Answer 0

#[test]
fn test_is_null_with_null_value() {
    let v = serde_json::json!(null);
    assert!(v.is_null());
}

#[test]
fn test_is_null_with_false_value() {
    let v = serde_json::json!(false);
    assert!(!v.is_null());
}

#[test]
fn test_is_null_with_true_value() {
    let v = serde_json::json!(true);
    assert!(!v.is_null());
}

#[test]
fn test_is_null_with_number_value() {
    let v = serde_json::json!(42);
    assert!(!v.is_null());
}

#[test]
fn test_is_null_with_string_value() {
    let v = serde_json::json!("some string");
    assert!(!v.is_null());
}

#[test]
fn test_is_null_with_empty_array() {
    let v = serde_json::json!([]);
    assert!(!v.is_null());
}

#[test]
fn test_is_null_with_empty_object() {
    let v = serde_json::json!({});
    assert!(!v.is_null());
}

#[test]
fn test_is_null_with_nested_null() {
    let v = serde_json::json!({"key": null});
    assert!(v["key"].is_null());
}

