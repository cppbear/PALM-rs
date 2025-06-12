// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let serializer = MapKeySerializer;
    let value = 0u32;
    let _ = serializer.serialize_u32(value);
}

#[test]
fn test_serialize_u32_one() {
    let serializer = MapKeySerializer;
    let value = 1u32;
    let _ = serializer.serialize_u32(value);
}

#[test]
fn test_serialize_u32_max() {
    let serializer = MapKeySerializer;
    let value = 4294967295u32;
    let _ = serializer.serialize_u32(value);
}

#[test]
fn test_serialize_u32_mid() {
    let serializer = MapKeySerializer;
    let value = 2147483648u32;
    let _ = serializer.serialize_u32(value);
}

#[test]
fn test_serialize_u32_large_number() {
    let serializer = MapKeySerializer;
    let value = 999999999u32;
    let _ = serializer.serialize_u32(value);
}

