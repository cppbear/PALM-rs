// Answer 0

#[derive(Debug)]
enum Content {
    None,
    Str(&'static str),
    String(String),
    Bytes(Vec<u8>),
    ByteBuf(Vec<u8>),
}

impl Content {
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
fn test_as_str_none() {
    let content_none = Content::None;
    assert_eq!(content_none.as_str(), None);
}

