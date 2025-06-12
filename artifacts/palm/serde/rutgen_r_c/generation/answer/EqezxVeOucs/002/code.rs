// Answer 0

#[test]
fn test_as_str_with_bytes() {
    let content = Content::Bytes(vec![104, 101, 108, 108, 111]); // "hello" in bytes
    assert_eq!(content.as_str(), Some("hello"));
}

#[test]
fn test_as_str_with_empty_bytes() {
    let content = Content::Bytes(vec![]);
    assert_eq!(content.as_str(), Some(""));
}

#[test]
fn test_as_str_with_byte_buf() {
    let content = Content::ByteBuf(vec![119, 111, 114, 108, 100]); // "world" in bytes
    assert_eq!(content.as_str(), Some("world"));
}

#[test]
fn test_as_str_with_empty_byte_buf() {
    let content = Content::ByteBuf(vec![]);
    assert_eq!(content.as_str(), Some(""));
}

#[test]
fn test_as_str_with_str() {
    let content = Content::Str("test");
    assert_eq!(content.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_string() {
    let content = Content::String(String::from("string test"));
    assert_eq!(content.as_str(), Some("string test"));
}

#[test]
fn test_as_str_with_non_str_content() {
    let content = Content::I32(42);
    assert_eq!(content.as_str(), None);
}

