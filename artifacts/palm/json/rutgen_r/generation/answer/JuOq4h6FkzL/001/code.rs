// Answer 0

#[test]
fn test_eq_u64_matches() {
    let value = serde_json::json!(42);
    assert_eq!(eq_u64(&value, 42), true);
}

#[test]
fn test_eq_u64_does_not_match() {
    let value = serde_json::json!(42);
    assert_eq!(eq_u64(&value, 41), false);
}

#[test]
fn test_eq_u64_none() {
    let value = serde_json::json!(null);
    assert_eq!(eq_u64(&value, 42), false);
}

#[test]
fn test_eq_u64_with_large_value() {
    let value = serde_json::json!(u64::MAX);
    assert_eq!(eq_u64(&value, u64::MAX), true);
}

#[test]
fn test_eq_u64_with_zero() {
    let value = serde_json::json!(0);
    assert_eq!(eq_u64(&value, 0), true);
}

#[test]
fn test_eq_u64_with_non_u64() {
    let value = serde_json::json!("not a number");
    assert_eq!(eq_u64(&value, 42), false);
}

