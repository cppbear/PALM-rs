// Answer 0

#[test]
fn test_from_f32_positive_finite() {
    let result = Number::from_f32(3.14);
    assert!(result.is_some());
    if let Some(number) = result {
        assert!(number.is_f64());
        assert_eq!(number.as_f64(), Some(3.14));
    }
}

#[test]
fn test_from_f32_negative_finite() {
    let result = Number::from_f32(-2.71);
    assert!(result.is_some());
    if let Some(number) = result {
        assert!(number.is_f64());
        assert_eq!(number.as_f64(), Some(-2.71));
    }
}

#[test]
fn test_from_f32_zero() {
    let result = Number::from_f32(0.0);
    assert!(result.is_some());
    if let Some(number) = result {
        assert!(number.is_f64());
        assert_eq!(number.as_f64(), Some(0.0));
    }
}

#[test]
fn test_from_f32_large_finite() {
    let result = Number::from_f32(1e10);
    assert!(result.is_some());
    if let Some(number) = result {
        assert!(number.is_f64());
        assert_eq!(number.as_f64(), Some(1e10));
    }
}

#[test]
fn test_from_f32_small_finite() {
    let result = Number::from_f32(1e-10);
    assert!(result.is_some());
    if let Some(number) = result {
        assert!(number.is_f64());
        assert_eq!(number.as_f64(), Some(1e-10));
    }
}

#[test]
fn test_from_f32_negative_large_finite() {
    let result = Number::from_f32(-1e10);
    assert!(result.is_some());
    if let Some(number) = result {
        assert!(number.is_f64());
        assert_eq!(number.as_f64(), Some(-1e10));
    }
}

