// Answer 0

#[test]
fn test_serialize_empty_string() {
    let serializer = Serializer;
    serializer.serialize_str("");
}

#[test]
fn test_serialize_simple_string() {
    let serializer = Serializer;
    serializer.serialize_str("hello");
}

#[test]
fn test_serialize_string_with_spaces() {
    let serializer = Serializer;
    serializer.serialize_str("hello world");
}

#[test]
fn test_serialize_string_with_special_chars() {
    let serializer = Serializer;
    serializer.serialize_str("string with special chars: !@#$%^&*()");
}

#[test]
fn test_serialize_unicode_string() {
    let serializer = Serializer;
    serializer.serialize_str("unicode test: 測試");
}

#[test]
fn test_serialize_string_with_newline() {
    let serializer = Serializer;
    serializer.serialize_str("first line\nsecond line");
}

#[test]
fn test_serialize_string_with_tab() {
    let serializer = Serializer;
    serializer.serialize_str("string with\ttab");
}

