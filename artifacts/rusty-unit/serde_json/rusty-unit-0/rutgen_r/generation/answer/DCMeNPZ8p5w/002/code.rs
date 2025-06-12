// Answer 0

#[test]
fn test_as_array_with_non_empty_array() {
    use serde_json::{json, Value};

    let v = json!([1, 2, 3]);
    assert_eq!(v.as_array().unwrap().len(), 3);
}

#[test]
fn test_as_array_with_empty_array() {
    use serde_json::{json, Value};

    let v = json!([]);
    assert_eq!(v.as_array().unwrap().len(), 0);
}

#[test]
fn test_as_array_with_nested_array() {
    use serde_json::{json, Value};

    let v = json!([["inner_array_1", "inner_array_2"], [3, 4]]);
    assert_eq!(v.as_array().unwrap().len(), 2);
    assert_eq!(v.as_array().unwrap()[0].as_array().unwrap().len(), 2);
}

#[test]
fn test_as_array_with_non_array_value() {
    use serde_json::{json, Value};

    let v = json!(42);
    assert_eq!(v.as_array(), None);
}

#[test]
fn test_as_array_with_object() {
    use serde_json::{json, Value};

    let v = json!({"key": "value"});
    assert_eq!(v.as_array(), None);
}

