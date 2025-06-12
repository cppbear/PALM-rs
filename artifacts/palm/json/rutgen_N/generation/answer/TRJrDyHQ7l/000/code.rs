// Answer 0

#[test]
fn test_is_string_with_string_value() {
    use serde_json::json;

    let v = json!({ "key": "string value" });
    assert!(v["key"].is_string());
}

#[test]
fn test_is_string_with_non_string_value() {
    use serde_json::json;

    let v = json!({ "key": 123 });
    assert!(!v["key"].is_string());
}

#[test]
fn test_is_string_with_boolean_value() {
    use serde_json::json;

    let v = json!({ "key": false });
    assert!(!v["key"].is_string());
}

#[test]
fn test_is_string_with_null_value() {
    use serde_json::json;

    let v = json!({ "key": null });
    assert!(!v["key"].is_string());
}

#[test]
fn test_is_string_with_empty_string() {
    use serde_json::json;

    let v = json!({ "key": "" });
    assert!(v["key"].is_string());
}

