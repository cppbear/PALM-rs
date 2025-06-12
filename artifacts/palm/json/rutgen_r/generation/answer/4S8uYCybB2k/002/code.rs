// Answer 0

#[test]
fn test_as_u64_with_valid_integer() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(64);
    assert_eq!(v.as_u64(), Some(64));
}

#[test]
fn test_as_u64_with_negative_integer() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(-64);
    assert_eq!(v.as_u64(), None);
}

#[test]
fn test_as_u64_with_float_number() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(256.0);
    assert_eq!(v.as_u64(), None);
}

#[test]
fn test_as_u64_with_large_integer() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(u64::MAX);
    assert_eq!(v.as_u64(), Some(u64::MAX));
}

#[test]
fn test_as_u64_with_zero() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(0);
    assert_eq!(v.as_u64(), Some(0));
}

#[test]
fn test_as_u64_with_non_number() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!("string");
    assert_eq!(v.as_u64(), None);

    let v = json!(true);
    assert_eq!(v.as_u64(), None);

    let v = json!(null);
    assert_eq!(v.as_u64(), None);
}

