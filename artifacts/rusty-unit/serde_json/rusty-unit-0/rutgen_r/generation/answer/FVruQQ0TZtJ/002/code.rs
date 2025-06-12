// Answer 0

#[test]
fn test_is_f64_with_f64_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(256.0);
    assert!(v.is_f64());
}

#[test]
fn test_is_f64_with_i64_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(64);
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_u64_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(64u64);
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_negative_i64_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(-64);
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_non_number_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!("string");
    assert!(!v.is_f64());

    let v = json!(true);
    assert!(!v.is_f64());

    let v = json!(null);
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_float_edge_cases() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(0.0);
    assert!(v.is_f64());

    let v = json!(f64::INFINITY);
    assert!(v.is_f64());

    let v = json!(f64::NAN);
    assert!(v.is_f64());
}

