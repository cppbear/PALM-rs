// Answer 0

#[test]
fn test_serialize_i128_positive() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i128(123456789012345678901234567890i128);
}

#[test]
fn test_serialize_i128_negative() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i128(-123456789012345678901234567890i128);
}

#[test]
fn test_serialize_i128_zero() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i128(0i128);
}

#[test]
fn test_serialize_i128_min_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i128(i128::MIN);
}

#[test]
fn test_serialize_i128_max_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer: &mut writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i128(i128::MAX);
}

