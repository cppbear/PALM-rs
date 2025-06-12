// Answer 0

#[test]
fn test_serialize_f32_nan() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    let result = map_key_serializer.serialize_f32(f32::NAN);
}

#[test]
fn test_serialize_f32_infinity() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    let result = map_key_serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_neg_infinity() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    let result = map_key_serializer.serialize_f32(f32::NEG_INFINITY);
}

