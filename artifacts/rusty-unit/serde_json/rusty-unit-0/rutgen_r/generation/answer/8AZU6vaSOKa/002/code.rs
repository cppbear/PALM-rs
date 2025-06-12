// Answer 0

#[test]
fn test_is_i64_with_positive_i64() {
    let v = serde_json::json!(42);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_negative_i64() {
    let v = serde_json::json!(-42);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_zero() {
    let v = serde_json::json!(0);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_large_i64() {
    let v = serde_json::json!(i64::MAX);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_large_unsigned() {
    let v = serde_json::json!(i64::MAX as u64 + 1);
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_with_decimal_value() {
    let v = serde_json::json!(3.14);
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_with_string_value() {
    let v = serde_json::json!("hello");
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_with_null_value() {
    let v = serde_json::json!(null);
    assert!(!v.is_i64());
}

