// Answer 0

#[test]
fn test_is_null_with_null_value() {
    use serde_json::json;

    let v = json!(null);
    assert!(v.is_null());
}

#[test]
fn test_is_null_with_non_null_value() {
    use serde_json::json;

    let v = json!(false);
    assert!(!v.is_null());

    let v_string = json!("string");
    assert!(!v_string.is_null());

    let v_number = json!(42);
    assert!(!v_number.is_null());

    let v_array = json!([1, 2, 3]);
    assert!(!v_array.is_null());

    let v_object = json!({"key": "value"});
    assert!(!v_object.is_null());
}

#[test]
fn test_is_null_with_nested_null() {
    use serde_json::json;

    let v = json!({"a": null, "b": 42});
    assert!(v["a"].is_null());
    assert!(!v["b"].is_null());
}

