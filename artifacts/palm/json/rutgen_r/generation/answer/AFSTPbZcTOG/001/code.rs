// Answer 0

#[test]
fn test_serialize_f32_positive_finite() {
    let value: f32 = 1.5;
    let result = serialize_f32(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1.5");
}

#[test]
fn test_serialize_f32_negative_finite() {
    let value: f32 = -3.14;
    let result = serialize_f32(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "-3.14");
}

#[test]
fn test_serialize_f32_zero() {
    let value: f32 = 0.0;
    let result = serialize_f32(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_serialize_f32_smallest_positive() {
    let value: f32 = std::f32::EPSILON;
    let result = serialize_f32(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1.1920929e-07");
}

#[test]
fn test_serialize_f32_large_finite() {
    let value: f32 = 1e38;
    let result = serialize_f32(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1e38");
}

