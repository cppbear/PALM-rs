// Answer 0

#[test]
fn test_from_f64_positive_finite() {
    let number = Number::from_f64(256.0).unwrap();
    assert!(number.is_f64());
    assert_eq!(number.as_f64(), Some(256.0));
}

#[test]
fn test_from_f64_negative_finite() {
    let number = Number::from_f64(-128.75).unwrap();
    assert!(number.is_f64());
    assert_eq!(number.as_f64(), Some(-128.75));
}

#[test]
fn test_from_f64_zero() {
    let number = Number::from_f64(0.0).unwrap();
    assert!(number.is_f64());
    assert_eq!(number.as_f64(), Some(0.0));
}

#[test]
fn test_from_f64_large_finite() {
    let number = Number::from_f64(1e10).unwrap();
    assert!(number.is_f64());
    assert_eq!(number.as_f64(), Some(1e10));
}

#[test]
fn test_from_f64_small_finite() {
    let number = Number::from_f64(1e-10).unwrap();
    assert!(number.is_f64());
    assert_eq!(number.as_f64(), Some(1e-10));
}

#[test]
fn test_from_f64_infinity() {
    let number = Number::from_f64(f64::INFINITY);
    assert!(number.is_none());
}

#[test]
fn test_from_f64_negative_infinity() {
    let number = Number::from_f64(f64::NEG_INFINITY);
    assert!(number.is_none());
}

#[test]
fn test_from_f64_nan() {
    let number = Number::from_f64(f64::NAN);
    assert!(number.is_none());
}

