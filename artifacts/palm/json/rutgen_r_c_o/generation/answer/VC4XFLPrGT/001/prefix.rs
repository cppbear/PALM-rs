// Answer 0

#[test]
fn test_serialize_i128_min_value() {
    let serializer = MapKeySerializer;
    let value = -9223372036854775808i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_negative_value() {
    let serializer = MapKeySerializer;
    let value = -1234567890123456789i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_zero() {
    let serializer = MapKeySerializer;
    let value = 0i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_positive_value() {
    let serializer = MapKeySerializer;
    let value = 1234567890123456789i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_max_value() {
    let serializer = MapKeySerializer;
    let value = 9223372036854775807i128;
    serializer.serialize_i128(value);
}

