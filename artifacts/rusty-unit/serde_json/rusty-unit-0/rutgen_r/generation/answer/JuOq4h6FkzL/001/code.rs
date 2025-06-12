// Answer 0

fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64() == Some(other)
}

#[test]
fn test_eq_u64_with_matching_value() {
    let value = serde_json::json!(42);
    let other = 42;
    assert!(eq_u64(&value, other));
}

#[test]
fn test_eq_u64_with_non_matching_value() {
    let value = serde_json::json!(42);
    let other = 43;
    assert!(!eq_u64(&value, other));
}

#[test]
fn test_eq_u64_with_none_value() {
    let value = serde_json::json!(null);
    let other = 42;
    assert!(!eq_u64(&value, other));
}

#[test]
fn test_eq_u64_with_zero() {
    let value = serde_json::json!(0);
    let other = 0;
    assert!(eq_u64(&value, other));
}

#[test]
fn test_eq_u64_with_large_value() {
    let value = serde_json::json!(u64::MAX);
    let other = u64::MAX;
    assert!(eq_u64(&value, other));
}

#[test]
fn test_eq_u64_with_an_invalid_u64_value() {
    let value = serde_json::json!("not a number");
    let other = 42;
    assert!(!eq_u64(&value, other));
}

