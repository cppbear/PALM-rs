// Answer 0

#[test]
fn test_serialize_str_empty_string() {
    let mut writer = Vec::new();
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_str("").unwrap();
}

#[test]
fn test_serialize_str_single_character() {
    let mut writer = Vec::new();
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_str("a").unwrap();
}

#[test]
fn test_serialize_str_max_length_string() {
    let mut writer = Vec::new();
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    let max_length_string = "a".repeat(255);
    key_serializer.serialize_str(&max_length_string).unwrap();
}

#[test]
fn test_serialize_str_valid_utf8_string() {
    let mut writer = Vec::new();
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_str("Hello, world!").unwrap();
}

#[test]
fn test_serialize_str_special_characters() {
    let mut writer = Vec::new();
    let mut serializer = Serializer { writer, formatter: CompactFormatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_str("!@#$%^&*()_+").unwrap();
}

