// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let serializer = MapKeySerializer;
    let value = 0u64;
    let _ = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_one() {
    let serializer = MapKeySerializer;
    let value = 1u64;
    let _ = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_max() {
    let serializer = MapKeySerializer;
    let value = u64::MAX; // 2^64 - 1
    let _ = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_large_value() {
    let serializer = MapKeySerializer;
    let value = 12345678901234567890u64; 
    let _ = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_mid_range() {
    let serializer = MapKeySerializer;
    let value = 1234567890u64;
    let _ = serializer.serialize_u64(value);
}

