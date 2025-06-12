// Answer 0

#[test]
fn test_as_str_byte_buf() {
    let content = Content::ByteBuf(vec![104, 101, 108, 108, 111]); // "hello" in bytes
    assert_eq!(content.as_str(), Some("hello"));
}

#[test]
fn test_as_str_bytes() {
    let content = Content::Bytes(b"world"); // "world" in bytes
    assert_eq!(content.as_str(), Some("world"));
}

#[test]
fn test_as_str_string() {
    let content = Content::String("serde".to_string());
    assert_eq!(content.as_str(), Some("serde"));
}

#[test]
fn test_as_str_str() {
    let content = Content::Str("test string");
    assert_eq!(content.as_str(), Some("test string"));
}

#[test]
fn test_as_str_none() {
    let content = Content::U32(42);
    assert_eq!(content.as_str(), None);
}

#[test]
fn test_as_str_unit() {
    let content = Content::Unit;
    assert_eq!(content.as_str(), None);
}

