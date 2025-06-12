// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_max() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_u8(255);
}

#[test]
fn test_serialize_u8_mid() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_u8(128);
}

#[test]
fn test_serialize_u8_small() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_large() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_u8(254);
}

#[test]
fn test_serialize_u8_random() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_u8(42);
}

