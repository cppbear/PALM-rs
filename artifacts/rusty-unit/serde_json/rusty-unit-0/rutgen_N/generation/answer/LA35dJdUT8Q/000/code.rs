// Answer 0

#[test]
fn test_as_object_with_valid_object() {
    use serde_json::{json, Value, Map};

    let v = json!({ "key": "value" });
    let result = v.as_object().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.get("key").unwrap(), &Value::String("value".to_string()));
}

#[test]
fn test_as_object_with_nested_object() {
    use serde_json::{json, Value};

    let v = json!({ "a": { "nested": true } });
    let result = v["a"].as_object().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.get("nested").unwrap(), &Value::Bool(true));
}

#[test]
fn test_as_object_with_array() {
    use serde_json::{json, Value};

    let v = json!([1, 2, 3]);
    let result = v.as_object();
    assert_eq!(result, None);
}

#[test]
fn test_as_object_with_string() {
    use serde_json::{json, Value};

    let v = json!("not an object");
    let result = v.as_object();
    assert_eq!(result, None);
}

#[test]
fn test_as_object_with_empty_object() {
    use serde_json::{json, Value};

    let v = json!({});
    let result = v.as_object().unwrap();
    assert_eq!(result.len(), 0);
}

