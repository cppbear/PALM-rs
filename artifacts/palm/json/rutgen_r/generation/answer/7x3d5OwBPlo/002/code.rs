// Answer 0

#[test]
fn test_as_number_with_u64() {
    use serde_json::{json, Number};

    let v = json!({ "number": 42 });
    assert_eq!(v["number"].as_number(), Some(&Number::from(42u64)));
}

#[test]
fn test_as_number_with_i64() {
    use serde_json::{json, Number};

    let v = json!({ "number": -15 });
    assert_eq!(v["number"].as_number(), Some(&Number::from(-15i64)));
}

#[test]
fn test_as_number_with_f64() {
    use serde_json::{json, Number};

    let v = json!({ "number": 3.14 });
    assert_eq!(v["number"].as_number(), Some(&Number::from_f64(3.14).unwrap()));
}

#[test]
fn test_as_number_with_zero() {
    use serde_json::{json, Number};

    let v = json!({ "number": 0 });
    assert_eq!(v["number"].as_number(), Some(&Number::from(0u64)));
}

#[test]
fn test_as_number_with_decimal_negative() {
    use serde_json::{json, Number};

    let v = json!({ "number": -2.71 });
    assert_eq!(v["number"].as_number(), Some(&Number::from_f64(-2.71).unwrap()));
}

