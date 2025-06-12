// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(0);
}

#[test]
fn test_serialize_u32_min() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(1);
}

#[test]
fn test_serialize_u32_mid() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(2147483648); // Midpoint
}

#[test]
fn test_serialize_u32_max() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(4294967295);
}

#[test]
fn test_serialize_u32_small() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(123);
}

#[test]
fn test_serialize_u32_large() {
    let serializer = Serializer;
    let _ = serializer.serialize_u32(4000000000);
}

