// Answer 0

#[test]
fn test_serialize_u16_min() {
    let value: u16 = 0;
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_max() {
    let value: u16 = 65535;
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_middle() {
    let value: u16 = 32768;
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_random() {
    let value: u16 = 12345;
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_u16(value);
}

