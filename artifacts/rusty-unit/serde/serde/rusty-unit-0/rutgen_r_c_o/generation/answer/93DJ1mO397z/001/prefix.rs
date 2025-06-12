// Answer 0

#[test]
fn test_serialize_i16_min() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i16(-32768);
}

#[test]
fn test_serialize_i16_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_positive() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i16(32767);
}

#[test]
fn test_serialize_i16_negative() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i16(-12345);
}

#[test]
fn test_serialize_i16_boundary() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i16(32767);
    let _ = serializer.serialize_i16(-32768);
}

