// Answer 0

#[test]
fn test_serialize_f32_positive() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(3.40282347e+38);
}

#[test]
fn test_serialize_f32_negative() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(-3.40282347e+38);
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(0.0);
}

#[test]
fn test_serialize_f32_small() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(1.23e-38);
}

#[test]
fn test_serialize_f32_large() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(1.23e+38);
}

#[test]
fn test_serialize_f32_negative_small() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(-1.23e-38);
}

#[test]
fn test_serialize_f32_nan() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(f32::NAN);
}

#[test]
fn test_serialize_f32_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f32(f32::NEG_INFINITY);
}

