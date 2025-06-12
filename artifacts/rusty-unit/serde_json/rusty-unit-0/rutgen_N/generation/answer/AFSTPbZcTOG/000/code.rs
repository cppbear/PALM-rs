// Answer 0

#[test]
fn test_serialize_finite_f32() {
    let result = serialize_f32(0.0_f32);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_negative_finite_f32() {
    let result = serialize_f32(-3.14_f32);
    assert_eq!(result, Ok("-3.14".to_string()));
}

#[test]
fn test_serialize_finite_large_f32() {
    let result = serialize_f32(1e38_f32);
    assert_eq!(result, Ok("1e38".to_string()));
}

#[test]
fn test_serialize_nan_f32() {
    let result = serialize_f32(f32::NAN);
    assert!(result.is_err());
}

#[test]
fn test_serialize_infinity_f32() {
    let result = serialize_f32(f32::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_serialize_negative_infinity_f32() {
    let result = serialize_f32(f32::NEG_INFINITY);
    assert!(result.is_err());
}

