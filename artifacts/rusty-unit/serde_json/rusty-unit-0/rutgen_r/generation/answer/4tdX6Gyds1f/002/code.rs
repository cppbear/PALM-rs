// Answer 0

#[test]
fn test_from_f32_negative_infinity() {
    let result = from_f32(f32::NEG_INFINITY);
    assert_eq!(result, None);
}

#[test]
fn test_from_f32_positive_infinity() {
    let result = from_f32(f32::INFINITY);
    assert_eq!(result, None);
}

#[test]
fn test_from_f32_nan() {
    let result = from_f32(f32::NAN);
    assert_eq!(result, None);
}

