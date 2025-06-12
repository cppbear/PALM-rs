// Answer 0

#[test]
fn test_as_number_with_u64() {
    use serde_json::{json, Number};

    let v = json!(1);
    assert_eq!(v.as_number(), Some(&Number::from(1u64)));
}

#[test]
fn test_as_number_with_i64() {
    use serde_json::{json, Number};

    let v = json!(-3);
    assert_eq!(v.as_number(), Some(&Number::from(-3i64)));
}

#[test]
fn test_as_number_with_f64() {
    use serde_json::{json, Number};

    let v = json!(2.2);
    assert_eq!(v.as_number(), Some(&Number::from_f64(2.2).unwrap()));
}

#[test]
fn test_as_number_with_f64_negative() {
    use serde_json::{json, Number};

    let v = json!(-5.0);
    assert_eq!(v.as_number(), Some(&Number::from_f64(-5.0).unwrap()));
}

#[test]
fn test_as_number_with_zero() {
    use serde_json::{json, Number};

    let v = json!(0);
    assert_eq!(v.as_number(), Some(&Number::from(0u64)));
}

#[test]
fn test_as_number_with_null() {
    use serde_json::{json};

    let v = json!(null);
    assert_eq!(v.as_number(), None);
}

#[test]
fn test_as_number_with_bool() {
    use serde_json::{json};

    let v = json!(true);
    assert_eq!(v.as_number(), None);
}

#[test]
fn test_as_number_with_string() {
    use serde_json::{json};

    let v = json!("4");
    assert_eq!(v.as_number(), None);
}

