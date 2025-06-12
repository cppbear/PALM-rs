// Answer 0

#[test]
fn test_serialize_char_null() {
    let serializer = MapKeySerializer;
    let _result = serializer.serialize_char('\u{0000}');
}

#[test]
fn test_serialize_char_non_unicode() {
    let serializer = MapKeySerializer;
    let _result = serializer.serialize_char('\u{D7FF}');
}

#[test]
fn test_serialize_char_private_use_area() {
    let serializer = MapKeySerializer;
    let _result = serializer.serialize_char('\u{E000}');
}

#[test]
fn test_serialize_char_high_surrogates() {
    let serializer = MapKeySerializer;
    let _result = serializer.serialize_char('\u{FFFF}');
}

#[test]
fn test_serialize_char_unicode_max() {
    let serializer = MapKeySerializer;
    let _result = serializer.serialize_char('\u{10FFFF}');
}

