// Answer 0

#[test]
fn test_from_f32_finite_positive() {
    let result = Number::from_f32(3.14);
    assert!(result.is_some());
    let number = result.unwrap();
    assert!(number.is_f64());
    assert_eq!(number.as_f64(), Some(3.14));
}

#[test]
fn test_from_f32_finite_negative() {
    let result = Number::from_f32(-2.71);
    assert!(result.is_some());
    let number = result.unwrap();
    assert!(number.is_f64());
    assert_eq!(number.as_f64(), Some(-2.71));
}

#[test]
fn test_from_f32_infinite() {
    let result = Number::from_f32(f32::INFINITY);
    assert!(result.is_none());
}

#[test]
fn test_from_f32_nan() {
    let result = Number::from_f32(f32::NAN);
    assert!(result.is_none());
}

