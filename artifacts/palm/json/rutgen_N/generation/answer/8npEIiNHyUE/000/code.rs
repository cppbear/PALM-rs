// Answer 0

#[test]
fn test_as_null_with_null_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({ "a": null });
    assert_eq!(v["a"].as_null(), Some(()));
}

#[test]
fn test_as_null_with_non_null_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({ "b": false });
    assert_eq!(v["b"].as_null(), None);
}

#[test]
fn test_as_null_with_integer_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({ "c": 42 });
    assert_eq!(v["c"].as_null(), None);
}

#[test]
fn test_as_null_with_string_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({ "d": "hello" });
    assert_eq!(v["d"].as_null(), None);
}

#[test]
fn test_as_null_with_empty_object() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({});
    assert_eq!(v["e"].as_null(), None);
}

