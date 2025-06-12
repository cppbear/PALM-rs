// Answer 0

#[test]
fn test_serialize_i16_min_value() {
    let serializer = MapKeySerializer;
    serializer.serialize_i16(-32768);
}

#[test]
fn test_serialize_i16_negative_value() {
    let serializer = MapKeySerializer;
    serializer.serialize_i16(-12345);
}

#[test]
fn test_serialize_i16_zero_value() {
    let serializer = MapKeySerializer;
    serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_positive_value() {
    let serializer = MapKeySerializer;
    serializer.serialize_i16(12345);
}

#[test]
fn test_serialize_i16_max_value() {
    let serializer = MapKeySerializer;
    serializer.serialize_i16(32767);
}

