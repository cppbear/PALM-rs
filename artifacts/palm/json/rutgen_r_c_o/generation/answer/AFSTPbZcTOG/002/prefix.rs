// Answer 0

#[test]
fn test_serialize_f32_nan() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_f32(f32::NAN);
}

#[test]
fn test_serialize_f32_infinity() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_f32(f32::NEG_INFINITY);
}

