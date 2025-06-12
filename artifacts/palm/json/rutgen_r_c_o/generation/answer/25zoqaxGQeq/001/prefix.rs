// Answer 0

#[test]
fn test_serialize_char_null() {
    let serializer = Serializer;
    serializer.serialize_char('\u{0000}');
}

#[test]
fn test_serialize_char_min_valid() {
    let serializer = Serializer;
    serializer.serialize_char('\u{0020}');
}

#[test]
fn test_serialize_char_max_valid() {
    let serializer = Serializer;
    serializer.serialize_char('\u{D7FF}');
}

#[test]
fn test_serialize_char_e000() {
    let serializer = Serializer;
    serializer.serialize_char('\u{E000}');
}

#[test]
fn test_serialize_char_max_valid_2() {
    let serializer = Serializer;
    serializer.serialize_char('\u{FFFF}');
}

#[test]
fn test_serialize_char_special() {
    let serializer = Serializer;
    serializer.serialize_char('!');
}

#[test]
fn test_serialize_char_unicode() {
    let serializer = Serializer;
    serializer.serialize_char('😃');
}

