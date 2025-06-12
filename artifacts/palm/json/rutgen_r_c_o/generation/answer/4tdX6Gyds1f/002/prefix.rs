// Answer 0

#[test]
fn test_from_f32_nan() {
    let result = Number::from_f32(f32::NAN);
}

#[test]
fn test_from_f32_infinity() {
    let result = Number::from_f32(f32::INFINITY);
}

#[test]
fn test_from_f32_neg_infinity() {
    let result = Number::from_f32(f32::NEG_INFINITY);
}

