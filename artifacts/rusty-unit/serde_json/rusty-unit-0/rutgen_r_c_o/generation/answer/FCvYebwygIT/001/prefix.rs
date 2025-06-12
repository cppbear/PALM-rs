// Answer 0

#[test]
fn test_serialize_i32_negative() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(-2147483648);
}

#[test]
fn test_serialize_i32_negative_one() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(-1);
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_positive_one() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(1);
}

#[test]
fn test_serialize_i32_positive() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_i32(2147483647);
}

