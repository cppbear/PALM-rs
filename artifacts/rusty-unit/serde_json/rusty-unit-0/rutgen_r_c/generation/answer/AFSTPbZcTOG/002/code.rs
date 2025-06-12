// Answer 0

#[test]
fn test_serialize_f32_infinite() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), float_key_must_be_finite());
}

#[test]
fn test_serialize_f32_nan() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f32(f32::NAN);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), float_key_must_be_finite());
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f32(f32::NEG_INFINITY);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), float_key_must_be_finite());
}

