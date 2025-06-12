// Answer 0

#[derive(Debug)]
enum Content {
    Str(&'static str),
    String(String),
    Bytes(Vec<u8>),
    ByteBuf(Vec<u8>),
    Other,
}

impl Content {
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Content::Str(x) => Some(x),
            Content::String(ref x) => Some(x),
            Content::Bytes(x) => std::str::from_utf8(&x).ok(),
            Content::ByteBuf(ref x) => std::str::from_utf8(&x).ok(),
            _ => None,
        }
    }
}

#[test]
fn test_as_str_with_str_variant() {
    let content = Content::Str("test");
    assert_eq!(content.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_string_variant() {
    let content = Content::String(String::from("test"));
    assert_eq!(content.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_bytes_variant() {
    let content = Content::Bytes(vec![116, 101, 115, 116]); // b"test"
    assert_eq!(content.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_byte_buf_variant() {
    let content = Content::ByteBuf(vec![116, 101, 115, 116]); // b"test"
    assert_eq!(content.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_other_variant() {
    let content = Content::Other;
    assert_eq!(content.as_str(), None);
}

#[test]
fn test_as_str_with_invalid_bytes() {
    let content = Content::Bytes(vec![255, 255, 255]); // Invalid UTF-8
    assert_eq!(content.as_str(), None);
}

