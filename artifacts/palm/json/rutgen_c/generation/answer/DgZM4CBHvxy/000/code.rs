// Answer 0

#[test]
fn test_take_replaces_value_with_null() {
    use serde_json::json;

    let mut v = json!({ "x": "y" });
    assert_eq!(v["x"].take(), json!("y"));
    assert_eq!(v, json!({ "x": null }));
}

#[test]
fn test_take_on_null_value() {
    use serde_json::json;

    let mut v = json!(null);
    assert_eq!(v.take(), json!(null));
    assert_eq!(v, json!(null));
}

#[test]
fn test_take_on_nested_object() {
    use serde_json::json;

    let mut v = json!({ "a": { "b": "c" } });
    assert_eq!(v["a"]["b"].take(), json!("c"));
    assert_eq!(v, json!({ "a": { "b": null } }));
}

#[test]
fn test_take_on_array_value() {
    use serde_json::json;

    let mut v = json!([1, 2, 3]);
    assert_eq!(v[1].take(), json!(2));
    assert_eq!(v, json!([1, null, 3]));
}

#[test]
fn test_take_on_string_value() {
    use serde_json::json;

    let mut v = json!("string value");
    assert_eq!(v.take(), json!("string value"));
    assert_eq!(v, json!(null));
}

