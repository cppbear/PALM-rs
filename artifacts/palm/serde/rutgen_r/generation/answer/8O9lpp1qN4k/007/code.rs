// Answer 0

#[derive(Debug)]
enum Content {
    Bytes(Vec<u8>),
    None,
    Some(Box<Content>),
    Unit,
    // Other variants are omitted for brevity.
}

#[derive(Debug)]
enum Unexpected {
    Bytes(Vec<u8>),
    Option,
    Unit,
    // Other variants are omitted for brevity.
}

impl Content {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Content::Bytes(ref b) => Unexpected::Bytes(b.clone()),
            Content::None => Unexpected::Option,
            Content::Unit => Unexpected::Unit,
            _ => panic!("Unexpected variant"),
        }
    }
}

#[test]
fn test_unexpected_with_bytes() {
    let content = Content::Bytes(vec![1, 2, 3, 4, 5]);
    if let Unexpected::Bytes(ref bytes) = content.unexpected() {
        assert_eq!(bytes, &vec![1, 2, 3, 4, 5]);
    } else {
        panic!("Expected Unexpected::Bytes");
    }
}

#[test]
fn test_unexpected_with_empty_bytes() {
    let content = Content::Bytes(vec![]);
    if let Unexpected::Bytes(ref bytes) = content.unexpected() {
        assert_eq!(bytes, &vec![]);
    } else {
        panic!("Expected Unexpected::Bytes");
    }
}

