// Answer 0

#[test]
fn test_serialize_str_empty_string() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_str("");
}

#[test]
fn test_serialize_str_single_character() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_str("a");
}

#[test]
fn test_serialize_str_long_string() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_str("this is a long string to test serialization");
}

#[test]
fn test_serialize_str_special_characters() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_str("string with special characters @#$%^&*()");
}

#[test]
fn test_serialize_str_unicode_characters() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_str("unicode string: 你好, привет, مرحبا");
}

