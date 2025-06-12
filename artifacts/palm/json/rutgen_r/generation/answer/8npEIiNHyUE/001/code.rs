// Answer 0

#[test]
fn test_as_null_none_with_boolean_false() {
    let v = serde_json::json!({ "a": false });
    assert_eq!(v["a"].as_null(), None);
}

#[test]
fn test_as_null_none_with_boolean_true() {
    let v = serde_json::json!({ "b": true });
    assert_eq!(v["b"].as_null(), None);
}

#[test]
fn test_as_null_none_with_integer() {
    let v = serde_json::json!({ "c": 42 });
    assert_eq!(v["c"].as_null(), None);
}

#[test]
fn test_as_null_none_with_string() {
    let v = serde_json::json!({ "d": "hello" });
    assert_eq!(v["d"].as_null(), None);
}

#[test]
fn test_as_null_none_with_object() {
    let v = serde_json::json!({ "e": { "nested": "object" } });
    assert_eq!(v["e"].as_null(), None);
}

#[test]
fn test_as_null_none_with_array() {
    let v = serde_json::json!({ "f": [1, 2, 3] });
    assert_eq!(v["f"].as_null(), None);
}

