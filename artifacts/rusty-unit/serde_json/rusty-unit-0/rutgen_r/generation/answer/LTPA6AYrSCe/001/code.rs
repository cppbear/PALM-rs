// Answer 0

#[test]
fn test_as_i64_non_number() {
    use serde_json::json;
    use serde_json::Value;

    let v1 = json!(true);
    let v2 = json!("string");
    let v3 = json!(null);
    let v4 = json!([1, 2, 3]);
    let v5 = json!({"key": "value"});

    assert_eq!(v1.as_i64(), None);
    assert_eq!(v2.as_i64(), None);
    assert_eq!(v3.as_i64(), None);
    assert_eq!(v4.as_i64(), None);
    assert_eq!(v5.as_i64(), None);
}

