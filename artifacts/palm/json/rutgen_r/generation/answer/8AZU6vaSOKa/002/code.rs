// Answer 0

#[test]
fn test_is_i64_with_positive_integer() {
    use serde_json::json;

    let v = json!(42);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_negative_integer() {
    use serde_json::json;

    let v = json!(-10);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_i64_max() {
    use serde_json::json;

    let v = json!(i64::MAX);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_i64_min() {
    use serde_json::json;

    let v = json!(i64::MIN);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_with_large_integer() {
    use serde_json::json;

    let v = json!(i64::MAX as u64 + 1); // Out of i64 range
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_with_non_integer_number() {
    use serde_json::json;

    let v = json!(256.0); // A decimal number
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_with_string() {
    use serde_json::json;

    let v = json!("string");
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_with_boolean_true() {
    use serde_json::json;

    let v = json!(true);
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_with_boolean_false() {
    use serde_json::json;

    let v = json!(false);
    assert!(!v.is_i64());
}

