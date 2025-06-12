// Answer 0

#[test]
fn test_eq_i64_with_matching_value() {
    use serde_json::Value;

    let value = Value::from(42);
    let other = 42;
    assert_eq!(eq_i64(&value, other), true);
}

#[test]
fn test_eq_i64_with_non_matching_value() {
    use serde_json::Value;

    let value = Value::from(42);
    let other = 100;
    assert_eq!(eq_i64(&value, other), false);
}

#[test]
fn test_eq_i64_with_none_value() {
    use serde_json::Value;

    let value = Value::Null;
    let other = 42;
    assert_eq!(eq_i64(&value, other), false);
}

#[test]
fn test_eq_i64_with_no_i64_value() {
    use serde_json::Value;

    let value = Value::from("string");
    let other = 42;
    assert_eq!(eq_i64(&value, other), false);
}

#[test]
fn test_eq_i64_with_large_i64() {
    use serde_json::Value;

    let value = Value::from(i64::MAX);
    let other = i64::MAX;
    assert_eq!(eq_i64(&value, other), true);
}

#[test]
fn test_eq_i64_with_small_i64() {
    use serde_json::Value;

    let value = Value::from(i64::MIN);
    let other = i64::MIN;
    assert_eq!(eq_i64(&value, other), true);
}

