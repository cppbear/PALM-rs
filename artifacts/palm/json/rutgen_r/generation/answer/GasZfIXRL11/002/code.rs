// Answer 0

#[test]
fn test_peek_invalid_type_null() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"null");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    assert!(result.is_ok);
}

#[test]
fn test_peek_invalid_type_true() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"true");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Bool(true), &exp));
}

#[test]
fn test_peek_invalid_type_false() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"false");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Bool(false), &exp));
}

#[test]
fn test_peek_invalid_type_negative_number() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"-42");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    // Assuming the way negative number is handled doesn't panic and returns an error
    assert!(result.is_ok);
}

#[test]
fn test_peek_invalid_type_positive_number() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"123");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    // Assuming that the positive number also doesn't panic and returns an error
    assert!(result.is_ok);
}

#[test]
fn test_peek_invalid_type_string() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"\"Hello\"");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Str("Hello"), &exp));
}

#[test]
fn test_peek_invalid_type_array() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"[1, 2, 3]");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Seq, &exp));
}

#[test]
fn test_peek_invalid_type_object() {
    struct MockExpected;
    let mut input = MyDeserializer::new(b"{\"key\": \"value\"}");
    let exp = MockExpected;
    let result = input.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Map, &exp));
}

