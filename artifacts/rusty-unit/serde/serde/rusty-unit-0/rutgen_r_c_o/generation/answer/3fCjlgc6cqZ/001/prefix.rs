// Answer 0

#[test]
fn test_serialize_u16_min() {
    let serializer = ContentSerializer::<()> { error: PhantomData };
    let _ = serializer.serialize_u16(0);
}

#[test]
fn test_serialize_u16_mid() {
    let serializer = ContentSerializer::<()> { error: PhantomData };
    let _ = serializer.serialize_u16(32768);
}

#[test]
fn test_serialize_u16_max() {
    let serializer = ContentSerializer::<()> { error: PhantomData };
    let _ = serializer.serialize_u16(65535);
}

