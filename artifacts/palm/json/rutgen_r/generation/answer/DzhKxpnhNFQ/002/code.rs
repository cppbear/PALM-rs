// Answer 0

#[test]
fn test_serialize_f64_nan() {
    let result = serialize_f64(0.0, f64::NAN);
    assert!(result.is_err());
}

#[test]
fn test_serialize_f64_infinity() {
    let result = serialize_f64(0.0, f64::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_serialize_f64_neg_infinity() {
    let result = serialize_f64(0.0, f64::NEG_INFINITY);
    assert!(result.is_err());
}

