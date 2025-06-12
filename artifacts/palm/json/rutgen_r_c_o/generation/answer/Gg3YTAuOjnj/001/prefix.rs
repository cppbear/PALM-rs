// Answer 0

#[test]
fn test_serialize_empty_string() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    serializer.serialize_str("").unwrap();
}

#[test]
fn test_serialize_string_with_newline() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    serializer.serialize_str("Hello\nWorld").unwrap();
}

#[test]
fn test_serialize_string_with_tab() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    serializer.serialize_str("Hello\tWorld").unwrap();
}

#[test]
fn test_serialize_string_with_carriage_return() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    serializer.serialize_str("Hello\rWorld").unwrap();
}

#[test]
fn test_serialize_string_with_backslash() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    serializer.serialize_str("Hello\\World").unwrap();
}

#[test]
fn test_serialize_string_with_all_escape_sequences() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    serializer.serialize_str("Hello\n\t\r\\\"").unwrap();
}

#[test]
fn test_serialize_maximum_length_string() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    let max_length_string = "x".repeat(1024);
    serializer.serialize_str(&max_length_string).unwrap();
}

#[test]
fn test_serialize_string_with_special_characters() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::default();
    let serializer = Serializer { writer, formatter };
    serializer.serialize_str("Special chars: !@#$%^&*()_+").unwrap();
}

