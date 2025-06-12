// Answer 0

#[test]
fn test_serialize_char_valid_character() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_char('a');
}

#[test]
fn test_serialize_char_valid_unicode_character() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_char('😊');
}

#[test]
fn test_serialize_char_min_value() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_char('\u{0}');
}

#[test]
fn test_serialize_char_max_value() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_char('\u{10FFFF}');
}

