// Answer 0

#[test]
fn test_is_i64_valid_integer() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(64);
    assert!(v.is_i64());
}

#[test]
fn test_is_i64_exceeds_i64_max() {
    use serde_json::json;
    use serde_json::Value;

    let big = i64::max_value() as u64 + 10;
    let v = json!(big);
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_decimal_number() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(256.0);
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_non_numeric_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!("string");
    assert!(!v.is_i64());
}

#[test]
fn test_is_i64_null_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(null);
    assert!(!v.is_i64());
}

