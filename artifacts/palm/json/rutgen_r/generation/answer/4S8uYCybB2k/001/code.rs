// Answer 0

#[test]
fn test_as_u64_with_non_number_value() {
    use serde_json::json;
    use serde_json::Value;

    let v1 = json!(true);
    let v2 = json!("string");
    let v3 = json!(null);
    let v4 = json!({ "key": "value" });
    let v5 = json!([1, 2, 3]);

    assert_eq!(v1.as_u64(), None);
    assert_eq!(v2.as_u64(), None);
    assert_eq!(v3.as_u64(), None);
    assert_eq!(v4.as_u64(), None);
    assert_eq!(v5.as_u64(), None);
}

