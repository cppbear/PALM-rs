// Answer 0

#[test]
fn test_as_i64_with_valid_i64() {
    use serde_json::json;
    let v = json!(64);
    assert_eq!(v.as_i64(), Some(64));
}

#[test]
fn test_as_i64_with_large_u64() {
    use serde_json::json;
    let big = i64::max_value() as u64 + 10;
    let v = json!(big);
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_floating_point() {
    use serde_json::json;
    let v = json!(256.0);
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_non_number() {
    use serde_json::json;
    let v = json!("string");
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_negative_i64() {
    use serde_json::json;
    let v = json!(-64);
    assert_eq!(v.as_i64(), Some(-64));
}

