// Answer 0

#[test]
fn test_as_i64_with_valid_integer() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(64);
    assert_eq!(v.as_i64(), Some(64));
}

#[test]
fn test_as_i64_with_large_number() {
    use serde_json::json;
    use serde_json::Value;

    let big = i64::max_value() as u64 + 10;
    let v = json!(big);
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_float() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(256.0);
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_negative_integer() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(-64);
    assert_eq!(v.as_i64(), Some(-64));
}

#[test]
fn test_as_i64_with_zero() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(0);
    assert_eq!(v.as_i64(), Some(0));
}

#[test]
fn test_as_i64_with_non_number_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!("string");
    assert_eq!(v.as_i64(), None);
}

