// Answer 0

#[test]
fn test_serialize_i16_positive() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i16(42);
    assert_eq!(result, Ok(Content::I16(42)));
}

#[test]
fn test_serialize_i16_negative() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i16(-42);
    assert_eq!(result, Ok(Content::I16(-42)));
}

#[test]
fn test_serialize_i16_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i16(0);
    assert_eq!(result, Ok(Content::I16(0)));
}

