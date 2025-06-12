// Answer 0

#[test]
fn test_unexpected_float_f32() {
    let content = Content::F32(3.14);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(3.14 as f64));
}

#[test]
fn test_unexpected_float_f32_zero() {
    let content = Content::F32(0.0);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(0.0 as f64));
}

#[test]
fn test_unexpected_float_f32_negative() {
    let content = Content::F32(-2.71);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(-2.71 as f64));
}

#[test]
fn test_unexpected_float_f32_infinity() {
    let content = Content::F32(std::f32::INFINITY);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(std::f32::INFINITY as f64));
}

