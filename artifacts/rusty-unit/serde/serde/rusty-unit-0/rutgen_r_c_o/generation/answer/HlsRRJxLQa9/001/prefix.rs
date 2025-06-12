// Answer 0

#[test]
fn test_serialize_i32_min_value() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(-2147483648);
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_max_value() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(2147483647);
}

