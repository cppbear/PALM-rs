// Answer 0

#[derive(Debug)]
enum Content<'a> {
    Str(&'a str),
    String(String),
    Bytes(&'a [u8]),
    ByteBuf(Vec<u8>),
    Other,
}

impl<'a> Content<'a> {
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Content::Str(x) => Some(x),
            Content::String(ref x) => Some(x),
            Content::Bytes(x) => std::str::from_utf8(x).ok(),
            Content::ByteBuf(ref x) => std::str::from_utf8(x).ok(),
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
    let content = Content::Bytes(b"test");
    assert_eq!(content.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_bytebuf_variant() {
    let content = Content::ByteBuf(vec![116, 101, 115, 116]); // ASCII for "test"
    assert_eq!(content.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_other_variant() {
    let content = Content::Other;
    assert_eq!(content.as_str(), None);
}

#[test]
fn test_as_str_with_invalid_utf8_bytes() {
    let invalid_bytes = [255, 255, 255];
    let content = Content::Bytes(&invalid_bytes);
    assert_eq!(content.as_str(), None);
}

