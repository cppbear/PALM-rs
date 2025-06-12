// Answer 0

#[test]
fn test_serialize_i8_min_value() {
    let serializer = MapKeySerializer;
    let value = -128i8;
    serializer.serialize_i8(value);
}

#[test]
fn test_serialize_i8_zero() {
    let serializer = MapKeySerializer;
    let value = 0i8;
    serializer.serialize_i8(value);
}

#[test]
fn test_serialize_i8_positive_range() {
    let serializer = MapKeySerializer;
    let value = 127i8;
    serializer.serialize_i8(value);
}

#[test]
fn test_serialize_i8_negative_range() {
    let serializer = MapKeySerializer;
    let value = -1i8;
    serializer.serialize_i8(value);
}

