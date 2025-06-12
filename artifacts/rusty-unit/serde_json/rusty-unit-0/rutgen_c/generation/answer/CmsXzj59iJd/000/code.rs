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
    assert_eq!(result, Ok(Value::Number((-42).into())));
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(0);
    assert_eq!(result, Ok(Value::Number(0.into())));
}

#[test]
#[should_panic]
fn test_serialize_i32_overflow() {
    let serializer = Serializer;
    let _result = serializer.serialize_i32(i32::MAX + 1);
}

#[test]
#[should_panic]
fn test_serialize_i32_underflow() {
    let serializer = Serializer;
    let _result = serializer.serialize_i32(i32::MIN - 1);
}

