// Answer 0

#[test]
fn test_eq_f64_with_matching_f64() {
    let value = serde_json::json!(3.14);
    let result = eq_f64(&value, 3.14);
    assert!(result);
}

#[test]
fn test_eq_f64_with_non_matching_f64() {
    let value = serde_json::json!(3.14);
    let result = eq_f64(&value, 2.71);
    assert!(!result);
}

#[test]
fn test_eq_f64_with_null_value() {
    let value = serde_json::json!(null);
    let result = eq_f64(&value, 0.0);
    assert!(!result);
}

#[test]
fn test_eq_f64_with_non_f64_value() {
    let value = serde_json::json!("string");
    let result = eq_f64(&value, 0.0);
    assert!(!result);
}

#[test]
fn test_eq_f64_with_f64_null() {
    let value = serde_json::json!(3.14);
    let result = eq_f64(&value, f64::NAN);
    assert!(!result);
}

#[test]
fn test_eq_f64_with_nan() {
    let value = serde_json::json!(f64::NAN);
    let result = eq_f64(&value, f64::NAN);
    assert!(!result);
}

