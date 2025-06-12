// Answer 0

#[test]
fn test_as_bool_non_boolean() {
    use serde_json::json;
    use serde_json::Value;

    let v1 = json!(42); // Integer value, should return None
    assert_eq!(v1.as_bool(), None);

    let v2 = json!("true"); // String value, should return None
    assert_eq!(v2.as_bool(), None);

    let v3 = json!(null); // Null value, should return None
    assert_eq!(v3.as_bool(), None);

    let v4 = json!([]); // Empty array, should return None
    assert_eq!(v4.as_bool(), None);

    let v5 = json!({}); // Empty object, should return None
    assert_eq!(v5.as_bool(), None);
}

