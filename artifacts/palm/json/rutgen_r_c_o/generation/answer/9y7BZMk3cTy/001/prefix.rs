// Answer 0

#[test]
fn test_serialize_i16_lower_bound() {
    let serializer = Serializer;
    let _ = serializer.serialize_i16(-32768);
}

#[test]
fn test_serialize_i16_negative_mid_range() {
    let serializer = Serializer;
    let _ = serializer.serialize_i16(-1000);
}

#[test]
fn test_serialize_i16_zero() {
    let serializer = Serializer;
    let _ = serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_positive_mid_range() {
    let serializer = Serializer;
    let _ = serializer.serialize_i16(1000);
}

#[test]
fn test_serialize_i16_upper_bound() {
    let serializer = Serializer;
    let _ = serializer.serialize_i16(32767);
}

