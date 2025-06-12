// Answer 0

#[test]
fn test_serialize_i16_min_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i16(-32768);
}

#[test]
fn test_serialize_i16_negative_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i16(-100);
}

#[test]
fn test_serialize_i16_zero_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_positive_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i16(100);
}

#[test]
fn test_serialize_i16_max_value() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i16(32767);
}

