// Answer 0

#[test]
fn test_from_f64_nan() {
    let result = from_f64(f64::NAN);
    assert!(result.is_none());
}

#[test]
fn test_from_f64_infinity() {
    let result = from_f64(f64::INFINITY);
    assert!(result.is_none());
}

#[test]
fn test_from_f64_negative_infinity() {
    let result = from_f64(f64::NEG_INFINITY);
    assert!(result.is_none());
}

