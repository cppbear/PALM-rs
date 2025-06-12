// Answer 0

#[test]
fn test_take_non_empty_object() {
    let mut v = serde_json::json!({ "x": "y" });
    assert_eq!(v["x"].take(), serde_json::json!("y"));
    assert_eq!(v, serde_json::json!({ "x": null }));
}

#[test]
fn test_take_empty_object() {
    let mut v = serde_json::json!({});
    assert_eq!(v.take(), serde_json::Value::Null);
    assert_eq!(v, serde_json::json!(null));
}

#[test]
fn test_take_array() {
    let mut v = serde_json::json!([1, 2, 3]);
    assert_eq!(v[0].take(), serde_json::json!(1));
    assert_eq!(v, serde_json::json!([null, 2, 3]));
}

#[test]
fn test_take_string() {
    let mut v = serde_json::json!("hello");
    assert_eq!(v.take(), serde_json::json!("hello"));
    assert_eq!(v, serde_json::json!(null));
}

#[test]
fn test_take_boolean() {
    let mut v = serde_json::json!(true);
    assert_eq!(v.take(), serde_json::json!(true));
    assert_eq!(v, serde_json::json!(null));
}

#[test]
fn test_take_null() {
    let mut v = serde_json::json!(null);
    assert_eq!(v.take(), serde_json::json!(null));
    assert_eq!(v, serde_json::json!(null));
}

