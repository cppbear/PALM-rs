// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u64(0);
}

#[test]
fn test_serialize_u64_max() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u64(18446744073709551615);
}

#[test]
fn test_serialize_u64_mid_range() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u64(9223372036854775808);
}

#[test]
fn test_serialize_u64_small_positive() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u64(12345);
}

#[test]
fn test_serialize_u64_large_positive() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u64(999999999999999999);
}

#[test]
fn test_serialize_u64_edge_case() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u64(18446744073709551614);
}

