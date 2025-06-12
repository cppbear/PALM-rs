// Answer 0

#[test]
fn test_as_number_with_unsigned_int() {
    use serde_json::{json, Number};

    let v = json!({ "a": 1 });
    assert_eq!(v["a"].as_number(), Some(&Number::from(1u64)));
}

#[test]
fn test_as_number_with_float() {
    use serde_json::{json, Number};

    let v = json!({ "b": 2.2 });
    assert_eq!(v["b"].as_number(), Some(&Number::from_f64(2.2).unwrap()));
}

#[test]
fn test_as_number_with_signed_int() {
    use serde_json::{json, Number};

    let v = json!({ "c": -3 });
    assert_eq!(v["c"].as_number(), Some(&Number::from(-3i64)));
}

#[test]
fn test_as_number_with_edge_case_zero() {
    use serde_json::{json, Number};

    let v = json!({ "d": 0 });
    assert_eq!(v["d"].as_number(), Some(&Number::from(0u64)));
}

#[test]
fn test_as_number_with_large_unsigned_int() {
    use serde_json::{json, Number};

    let v = json!({ "e": 12345678901234567890u64 });
    assert_eq!(v["e"].as_number(), Some(&Number::from(12345678901234567890u64)));
}

#[test]
fn test_as_number_with_large_signed_int() {
    use serde_json::{json, Number};

    let v = json!({ "f": -12345678901234567890i64 });
    assert_eq!(v["f"].as_number(), Some(&Number::from(-12345678901234567890i64)));
}

