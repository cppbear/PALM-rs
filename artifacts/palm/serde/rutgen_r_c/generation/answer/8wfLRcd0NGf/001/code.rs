// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u32(0);
    assert_eq!(result, Ok(Content::U32(0)));
}

#[test]
fn test_serialize_u32_positive() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u32(42);
    assert_eq!(result, Ok(Content::U32(42)));
}

#[test]
fn test_serialize_u32_max() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u32(u32::MAX);
    assert_eq!(result, Ok(Content::U32(u32::MAX)));
}

