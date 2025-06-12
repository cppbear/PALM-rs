// Answer 0

#[test]
fn test_serialize_f32_infinity() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_nan() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_f32(f32::NAN);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_f32(f32::NEG_INFINITY);
}

#[test]
fn test_serialize_f32_zero() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_f32(0.0);
}

#[test]
fn test_serialize_f32_positive_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_f32(1.23);
}

#[test]
fn test_serialize_f32_negative_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_f32(-1.23);
}

