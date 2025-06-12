// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let serializer = ContentSerializer::<()>::default();
    serializer.serialize_u32(0);
}

#[test]
fn test_serialize_u32_one() {
    let serializer = ContentSerializer::<()>::default();
    serializer.serialize_u32(1);
}

#[test]
fn test_serialize_u32_max() {
    let serializer = ContentSerializer::<()>::default();
    serializer.serialize_u32(4_294_967_295);
}

#[test]
fn test_serialize_u32_large() {
    let serializer = ContentSerializer::<()>::default();
    serializer.serialize_u32(2_147_483_648);
}

#[test]
fn test_serialize_u32_middle() {
    let serializer = ContentSerializer::<()>::default();
    serializer.serialize_u32(3_000_000_000);
}

