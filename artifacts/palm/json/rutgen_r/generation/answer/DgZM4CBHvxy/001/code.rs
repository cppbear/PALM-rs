// Answer 0

#[test]
fn test_take_from_non_empty_value() {
    use serde_json::json;
    use serde_json::Value;

    let mut v = json!({ "key": "value" });
    assert_eq!(v["key"].take(), json!("value"));
    assert_eq!(v, json!({ "key": null }));
}

#[test]
fn test_take_from_empty_value() {
    use serde_json::json;
    use serde_json::Value;

    let mut v = json!({});
    assert_eq!(v["key"].take(), Value::Null);
    assert_eq!(v, json!({ "key": null }));
}

#[test]
fn test_take_from_array_value() {
    use serde_json::json;
    use serde_json::Value;

    let mut v = json!([1, 2, 3]);
    assert_eq!(v[0].take(), json!(1));
    assert_eq!(v, json!([null, 2, 3]));
}

#[test]
fn test_take_from_nested_value() {
    use serde_json::json;
    use serde_json::Value;

    let mut v = json!({ "x": { "y": "z" } });
    assert_eq!(v["x"]["y"].take(), json!("z"));
    assert_eq!(v, json!({ "x": { "y": null } }));
}

#[test]
fn test_take_from_null_value() {
    use serde_json::json;
    use serde_json::Value;

    let mut v = json!(null);
    assert_eq!(v.take(), Value::Null);
}

