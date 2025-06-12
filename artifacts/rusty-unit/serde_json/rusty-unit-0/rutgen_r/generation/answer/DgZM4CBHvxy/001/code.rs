// Answer 0

#[test]
fn test_take_existing_value() {
    use serde_json::{json, Value};

    let mut v = json!({ "key": "value" });
    assert_eq!(v["key"].take(), json!("value"));
    assert_eq!(v, json!({ "key": null }));
}

#[test]
fn test_take_empty_object() {
    use serde_json::{json, Value};

    let mut v = json!({});
    assert_eq!(v["missing"].take(), Value::Null);
    assert_eq!(v, json!({ "missing": null }));
}

#[test]
fn test_take_nested_value() {
    use serde_json::{json, Value};

    let mut v = json!({ "outer": { "inner": "data" } });
    assert_eq!(v["outer"]["inner"].take(), json!("data"));
    assert_eq!(v, json!({ "outer": { "inner": null } }));
}

#[test]
fn test_take_non_existent_key() {
    use serde_json::{json, Value};

    let mut v = json!({ "key": "value" });
    assert_eq!(v["nonexistent"].take(), Value::Null);
    assert_eq!(v, json!({ "key": "value", "nonexistent": null }));
}

#[test]
fn test_take_value_already_null() {
    use serde_json::{json, Value};

    let mut v = json!({ "key": null });
    assert_eq!(v["key"].take(), Value::Null);
    assert_eq!(v, json!({ "key": null }));
}

