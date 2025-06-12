// Answer 0

#[test]
fn test_serialize_f32_with_finite_value() {
    let value: f32 = 1.23;
    let result = serialize_f32(value);
    assert_eq!(result, Ok("1.23".to_string()));
}

#[test]
fn test_serialize_f32_with_negative_finite_value() {
    let value: f32 = -4.56;
    let result = serialize_f32(value);
    assert_eq!(result, Ok("-4.56".to_string()));
}

#[test]
fn test_serialize_f32_with_zero() {
    let value: f32 = 0.0;
    let result = serialize_f32(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_f32_with_small_value() {
    let value: f32 = 1e-10;
    let result = serialize_f32(value);
    assert_eq!(result, Ok("1e-10".to_string()));
}

#[test]
fn test_serialize_f32_with_large_value() {
    let value: f32 = 3.4e38; // Maximum finite f32 value
    let result = serialize_f32(value);
    assert_eq!(result, Ok("3.4e38".to_string()));
}

