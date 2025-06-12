// Answer 0

#[test]
fn test_is_string_with_string_value() {
    use serde_json::json;
    
    let v = json!("some string");
    assert!(v.is_string());
}

#[test]
fn test_is_string_with_integer_value() {
    use serde_json::json;

    let v = json!(42);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_boolean_value() {
    use serde_json::json;

    let v = json!(false);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_null_value() {
    use serde_json::json;

    let v = json!(null);
    assert!(!v.is_string());
}

#[test]
fn test_is_string_with_empty_string() {
    use serde_json::json;

    let v = json!("");
    assert!(v.is_string());
}

#[test]
fn test_is_string_with_nested_object() {
    use serde_json::json;

    let v = json!({ "key": "value" });
    assert!(v["key"].is_string());
    assert!(!v["nonexistent"].is_string());
}

#[test]
fn test_is_string_with_array() {
    use serde_json::json;

    let v = json!(["value", "another"]);
    assert!(v[0].is_string());
    assert!(!v[1].is_string());
}

