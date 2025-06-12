// Answer 0

#[test]
fn test_as_f64_with_positive_f64() {
    use serde_json::json;
    
    let v = json!(256.0);
    assert_eq!(v.as_f64(), Some(256.0));
}

#[test]
fn test_as_f64_with_negative_f64() {
    use serde_json::json;

    let v = json!(-64.0);
    assert_eq!(v.as_f64(), Some(-64.0));
}

#[test]
fn test_as_f64_with_integer() {
    use serde_json::json;

    let v = json!(64);
    assert_eq!(v.as_f64(), Some(64.0));
}

#[test]
fn test_as_f64_with_zero() {
    use serde_json::json;

    let v = json!(0);
    assert_eq!(v.as_f64(), Some(0.0));
}

#[test]
fn test_as_f64_with_large_integer() {
    use serde_json::json;

    let v = json!(1_000_000);
    assert_eq!(v.as_f64(), Some(1_000_000.0));
}

#[test]
fn test_as_f64_with_large_f64() {
    use serde_json::json;

    let v = json!(1e10);
    assert_eq!(v.as_f64(), Some(1e10));
}

#[test]
fn test_as_f64_with_invalid_value() {
    use serde_json::json;

    let v = json!("not a number");
    assert_eq!(v.as_f64(), None);
}

#[test]
fn test_as_f64_with_boolean() {
    use serde_json::json;

    let v = json!(true);
    assert_eq!(v.as_f64(), None);
}

#[test]
fn test_as_f64_with_array() {
    use serde_json::json;

    let v = json!([1, 2, 3]);
    assert_eq!(v.as_f64(), None);
}

#[test]
fn test_as_f64_with_object() {
    use serde_json::json;

    let v = json!({"key": "value"});
    assert_eq!(v.as_f64(), None);
}

