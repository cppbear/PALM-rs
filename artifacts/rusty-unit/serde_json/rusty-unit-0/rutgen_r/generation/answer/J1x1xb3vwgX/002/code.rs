// Answer 0

#[test]
fn test_as_str_with_valid_string() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!("some string");
    assert_eq!(v.as_str(), Some("some string"));
}

#[test]
fn test_as_str_with_empty_string() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(""); // Testing with an empty string
    assert_eq!(v.as_str(), Some(""));
}

#[test]
fn test_as_str_with_false_boolean() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(false);
    assert_eq!(v.as_str(), None);
}

#[test]
fn test_as_str_with_number() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(42);
    assert_eq!(v.as_str(), None);
}

#[test]
fn test_as_str_with_null() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(null);
    assert_eq!(v.as_str(), None);
}

#[test]
fn test_as_str_with_object() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({"key": "value"});
    assert_eq!(v.as_str(), None);
}

