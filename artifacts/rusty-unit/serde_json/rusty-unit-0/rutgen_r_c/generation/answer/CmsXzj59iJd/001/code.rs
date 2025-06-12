// Answer 0

#[test]
fn test_serialize_i32_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(42);
    assert_eq!(result, Ok(Value::Number(42.into())));
}

#[test]
fn test_serialize_i32_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(-42);
    assert_eq!(result, Ok(Value::Number((-42i64).into())));
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(0);
    assert_eq!(result, Ok(Value::Number(0.into())));
}

#[test]
fn test_serialize_i32_max() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(i32::MAX);
    assert_eq!(result, Ok(Value::Number(i32::MAX as i64.into())));
}

#[test]
fn test_serialize_i32_min() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(i32::MIN);
    assert_eq!(result, Ok(Value::Number(i32::MIN as i64.into())));
}

