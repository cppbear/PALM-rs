// Answer 0

#[test]
fn test_as_i64_with_non_number() {
    use serde_json::json;
    use serde_json::Value;

    let v_string = json!("string");
    let v_bool = json!(true);
    let v_array = json!([1, 2, 3]);
    let v_object = json!({"key": "value"});
    let v_null = json!(null);

    assert_eq!(v_string.as_i64(), None);
    assert_eq!(v_bool.as_i64(), None);
    assert_eq!(v_array.as_i64(), None);
    assert_eq!(v_object.as_i64(), None);
    assert_eq!(v_null.as_i64(), None);
}

#[test]
fn test_as_i64_with_number() {
    use serde_json::json;
    use serde_json::Value;

    let v_number = json!(100);
    let v_float = json!(100.5);

    assert_eq!(v_number.as_i64(), Some(100));
    assert_eq!(v_float.as_i64(), None);
}

