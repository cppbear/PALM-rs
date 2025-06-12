// Answer 0

#[test]
fn test_take_removes_value_and_replaces_with_null() {
    use serde_json::{json, Value};

    let mut v = json!({ "x": "y" });
    assert_eq!(v["x"].take(), json!("y"));
    assert_eq!(v, json!({ "x": Value::Null }));
}

#[test]
fn test_take_on_empty_value() {
    use serde_json::{json, Value};

    let mut v = Value::Null;
    assert_eq!(v.take(), Value::Null);
    assert_eq!(v, Value::Null);
}

