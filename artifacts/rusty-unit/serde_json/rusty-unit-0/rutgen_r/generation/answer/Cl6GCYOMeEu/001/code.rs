// Answer 0

#[test]
fn test_eq_i64_with_matching_value() {
    let value = serde_json::json!(42);
    let other = 42;
    assert!(eq_i64(&value, other));
}

#[test]
fn test_eq_i64_with_non_matching_value() {
    let value = serde_json::json!(42);
    let other = 43;
    assert!(!eq_i64(&value, other));
}

#[test]
fn test_eq_i64_with_none_value() {
    let value = serde_json::json!(null);
    let other = 0;
    assert!(!eq_i64(&value, other));
}

#[test]
fn test_eq_i64_with_negative_value() {
    let value = serde_json::json!(-10);
    let other = -10;
    assert!(eq_i64(&value, other));
}

#[test]
fn test_eq_i64_with_large_value() {
    let value = serde_json::json!(i64::MAX);
    let other = i64::MAX;
    assert!(eq_i64(&value, other));
}

#[test]
fn test_eq_i64_with_large_non_matching_value() {
    let value = serde_json::json!(i64::MAX);
    let other = i64::MAX - 1;
    assert!(!eq_i64(&value, other));
}

