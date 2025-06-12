// Answer 0

#[test]
fn test_serialize_f32_nan() {
    let nan_value = f32::NAN;
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f32(nan_value);
}

#[test]
fn test_serialize_f32_infinity() {
    let infinity_value = f32::INFINITY;
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f32(infinity_value);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let negative_infinity_value = f32::NEG_INFINITY;
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f32(negative_infinity_value);
}

