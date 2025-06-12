// Answer 0

#[test]
fn test_serialize_i64_positive() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i64(42);
    assert_eq!(result, Ok(Content::I64(42)));
}

#[test]
fn test_serialize_i64_negative() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i64(-42);
    assert_eq!(result, Ok(Content::I64(-42)));
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i64(0);
    assert_eq!(result, Ok(Content::I64(0)));
}

