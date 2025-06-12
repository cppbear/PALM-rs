// Answer 0

#[test]
fn test_serialize_i64_min() {
    let serializer = ContentSerializer::<()>::new();
    let _ = serializer.serialize_i64(-9223372036854775808);
}

#[test]
fn test_serialize_i64_negative() {
    let serializer = ContentSerializer::<()>::new();
    let _ = serializer.serialize_i64(-1);
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = ContentSerializer::<()>::new();
    let _ = serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_positive() {
    let serializer = ContentSerializer::<()>::new();
    let _ = serializer.serialize_i64(1);
}

#[test]
fn test_serialize_i64_max() {
    let serializer = ContentSerializer::<()>::new();
    let _ = serializer.serialize_i64(9223372036854775807);
}

