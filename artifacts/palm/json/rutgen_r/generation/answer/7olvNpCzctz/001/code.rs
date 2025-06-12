// Answer 0

#[test]
fn test_serialize_f32_positive() {
    let float_value: f32 = 3.14;
    let result = serialize_f32(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f32_zero() {
    let float_value: f32 = 0.0;
    let result = serialize_f32(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f32_negative() {
    let float_value: f32 = -1.23;
    let result = serialize_f32(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f32_infinity() {
    let float_value: f32 = f32::INFINITY;
    let result = serialize_f32(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f32_neg_infinity() {
    let float_value: f32 = f32::NEG_INFINITY;
    let result = serialize_f32(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f32_nan() {
    let float_value: f32 = f32::NAN;
    let result = serialize_f32(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

