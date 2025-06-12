// Answer 0

#[test]
fn test_serialize_f32_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(0.0);
    assert_eq!(result, Ok(Content::F32(0.0)));
}

#[test]
fn test_serialize_f32_positive() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(1.23);
    assert_eq!(result, Ok(Content::F32(1.23)));
}

#[test]
fn test_serialize_f32_negative() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(-4.56);
    assert_eq!(result, Ok(Content::F32(-4.56)));
}

#[test]
fn test_serialize_f32_large() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(std::f32::MAX);
    assert_eq!(result, Ok(Content::F32(std::f32::MAX)));
}

#[test]
fn test_serialize_f32_small() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(std::f32::MIN);
    assert_eq!(result, Ok(Content::F32(std::f32::MIN)));
}

#[test]
fn test_serialize_f32_nan() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(std::f32::NAN);
    assert_eq!(result, Ok(Content::F32(std::f32::NAN)));
} 

#[test]
fn test_serialize_f32_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(std::f32::INFINITY);
    assert_eq!(result, Ok(Content::F32(std::f32::INFINITY)));
}

