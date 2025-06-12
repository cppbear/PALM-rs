// Answer 0

#[test]
fn test_from_f64_nan() {
    let result = Number::from_f64(f64::NAN);
    assert!(result.is_none());
}

#[test]
fn test_from_f64_infinity() {
    let result_positive = Number::from_f64(f64::INFINITY);
    assert!(result_positive.is_none());

    let result_negative = Number::from_f64(f64::NEG_INFINITY);
    assert!(result_negative.is_none());
}

