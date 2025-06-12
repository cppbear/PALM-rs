// Answer 0

#[test]
fn test_serialize_finite_f32_positive() {
    let serializer = MapKeySerializer;
    let value = 1.0f32;
    let _ = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_finite_f32_negative() {
    let serializer = MapKeySerializer;
    let value = -1.0f32;
    let _ = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_small_positive_f32() {
    let serializer = MapKeySerializer;
    let value = 0.0001f32;
    let _ = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_small_negative_f32() {
    let serializer = MapKeySerializer;
    let value = -0.0001f32;
    let _ = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_large_positive_f32() {
    let serializer = MapKeySerializer;
    let value = 3.4028235E38f32;
    let _ = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_large_negative_f32() {
    let serializer = MapKeySerializer;
    let value = -3.4028235E38f32;
    let _ = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_f32_boundary() {
    let serializer = MapKeySerializer;
    let value = 3.4E38f32; // close to the upper limit
    let _ = serializer.serialize_f32(value);
}

