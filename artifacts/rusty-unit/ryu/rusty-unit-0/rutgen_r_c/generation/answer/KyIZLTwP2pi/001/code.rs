// Answer 0

#[test]
fn test_is_nonfinite_with_nan() {
    let value = f64::NAN;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_positive_infinity() {
    let value = f64::INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_negative_infinity() {
    let value = f64::NEG_INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_finite_value() {
    let value = 3.14;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_zero() {
    let value = 0.0;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_negative_zero() {
    let value = -0.0;
    assert!(!value.is_nonfinite());
}

