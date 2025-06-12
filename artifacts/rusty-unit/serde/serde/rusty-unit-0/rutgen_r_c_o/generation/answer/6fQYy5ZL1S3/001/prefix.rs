// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(0);
}

#[test]
fn test_serialize_u64_minimum() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(1);
}

#[test]
fn test_serialize_u64_middle() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(9223372036854775808); // middle value
}

#[test]
fn test_serialize_u64_maximum() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(18446744073709551615); // maximum value
}

#[test]
fn test_serialize_u64_smallest_non_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(2);
}

