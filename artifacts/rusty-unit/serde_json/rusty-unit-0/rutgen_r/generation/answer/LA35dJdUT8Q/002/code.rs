// Answer 0

#[test]
fn test_as_object_with_simple_object() {
    use serde_json::{json, Map, Value};

    let v = json!({ "key1": "value1", "key2": "value2" });
    let result = v.as_object().unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("key1"), Some(&Value::from("value1")));
    assert_eq!(result.get("key2"), Some(&Value::from("value2")));
}

#[test]
fn test_as_object_with_nested_object() {
    use serde_json::{json, Map, Value};

    let v = json!({ "outer": { "inner": "value" } });
    let result = v.as_object().unwrap();
    assert_eq!(result.len(), 1);
    let inner_map = result.get("outer").unwrap().as_object().unwrap();
    assert_eq!(inner_map.len(), 1);
    assert_eq!(inner_map.get("inner"), Some(&Value::from("value")));
}

#[test]
fn test_as_object_with_empty_object() {
    use serde_json::{json, Map, Value};

    let v = json!({});
    let result = v.as_object().unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_as_object_with_non_object() {
    use serde_json::{json, Value};

    let v = json!(true);
    let result = v.as_object();
    assert_eq!(result, None);
}

#[test]
fn test_as_object_with_array() {
    use serde_json::{json, Value};

    let v = json!([1, 2, 3]);
    let result = v.as_object();
    assert_eq!(result, None);
}

