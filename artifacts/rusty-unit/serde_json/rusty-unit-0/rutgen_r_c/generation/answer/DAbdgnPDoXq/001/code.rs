// Answer 0

#[test]
fn test_serialize_char_valid() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_char('a');
    assert_eq!(result, Ok("a".to_string()));
}

#[test]
fn test_serialize_char_unicode() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_char('✓');
    assert_eq!(result, Ok("✓".to_string()));
}

#[test]
fn test_serialize_char_empty() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_char('\0'); // null character
    assert_eq!(result, Ok("\0".to_string()));
}

