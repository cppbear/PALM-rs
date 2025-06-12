// Answer 0

#[test]
fn test_as_u64_with_positive_integer() {
    use serde_json::json;
    let v = json!(64);
    assert_eq!(v.as_u64(), Some(64));
}

#[test]
fn test_as_u64_with_negative_integer() {
    use serde_json::json;
    let v = json!(-64);
    assert_eq!(v.as_u64(), None);
}

#[test]
fn test_as_u64_with_floating_point() {
    use serde_json::json;
    let v = json!(256.0);
    assert_eq!(v.as_u64(), None);
}

#[test]
fn test_as_u64_with_non_number() {
    use serde_json::json;
    let v = json!("string");
    assert_eq!(v.as_u64(), None);
}

#[test]
fn test_as_u64_with_null() {
    use serde_json::json;
    let v = json!(null);
    assert_eq!(v.as_u64(), None);
}

