// Answer 0

#[test]
fn test_serialize_f32_normal_range_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(1.23);
}

#[test]
fn test_serialize_f32_normal_range_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-1.23);
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(0.0);
}

#[test]
fn test_serialize_f32_large_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(3.4028235E38);
}

#[test]
fn test_serialize_f32_large_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-3.4028235E38);
}

#[test]
fn test_serialize_f32_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::NEG_INFINITY);
}

#[test]
fn test_serialize_f32_nan() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::NAN);
}

