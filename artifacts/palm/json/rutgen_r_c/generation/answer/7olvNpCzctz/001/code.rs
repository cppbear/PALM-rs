// Answer 0

#[test]
fn test_serialize_f32_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(1.23);
    assert_eq!(result, Ok(Value::Number(Number::from(1.23))));
}

#[test]
fn test_serialize_f32_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-1.23);
    assert_eq!(result, Ok(Value::Number(Number::from(-1.23))));
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(0.0);
    assert_eq!(result, Ok(Value::Number(Number::from(0.0))));
}

#[test]
fn test_serialize_f32_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::INFINITY);
    assert_eq!(result, Ok(Value::Number(Number::from(f32::INFINITY))));
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::NEG_INFINITY);
    assert_eq!(result, Ok(Value::Number(Number::from(f32::NEG_INFINITY))));
}

#[test]
fn test_serialize_f32_nan() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::NAN);
    assert_eq!(result, Ok(Value::Number(Number::from(f32::NAN))));
}

