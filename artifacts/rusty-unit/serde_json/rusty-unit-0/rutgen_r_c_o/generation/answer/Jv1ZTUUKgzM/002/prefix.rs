// Answer 0

#[test]
fn test_serialize_f32_infinity() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    
    let _ = serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    
    let _ = serializer.serialize_f32(f32::NEG_INFINITY);
}

#[test]
fn test_serialize_f32_nan() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    
    let _ = serializer.serialize_f32(f32::NAN);
}

