// Answer 0

#[test]
fn test_serialize_f32_positive() {
    let value: f32 = 1.0;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_f32(value);
}

#[test]
fn test_serialize_f32_negative() {
    let value: f32 = -1.0;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_f32(value);
}

#[test]
fn test_serialize_f32_zero() {
    let value: f32 = 0.0;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_f32(value);
}

#[test]
fn test_serialize_f32_large() {
    let value: f32 = 1e38;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_f32(value);
}

#[test]
fn test_serialize_f32_small() {
    let value: f32 = 1e-38;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_f32(value);
}

