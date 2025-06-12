// Answer 0

#[test]
fn test_as_null_with_non_null_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({ "b": false, "c": 42, "d": "string" });

    assert_eq!(v["b"].as_null(), None);
    assert_eq!(v["c"].as_null(), None);
    assert_eq!(v["d"].as_null(), None);
}

#[test]
fn test_as_null_with_nested_non_null() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({ "a": { "nested": false }, "b": [1, 2, 3] });

    assert_eq!(v["a"].as_null(), None);
    assert_eq!(v["b"].as_null(), None);
}

#[test]
fn test_as_null_with_array() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!([1, 2, null, false]);

    assert_eq!(v[0].as_null(), None);
    assert_eq!(v[1].as_null(), None);
    assert_eq!(v[3].as_null(), None);
}

