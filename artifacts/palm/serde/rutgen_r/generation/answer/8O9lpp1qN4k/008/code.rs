// Answer 0

#[derive(Debug)]
enum Content {
    ByteBuf(Vec<u8>),
    // Other variants omitted for brevity
}

#[derive(Debug)]
enum Unexpected {
    Bytes(Vec<u8>),
    // Other variants omitted for brevity
}

impl Content {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Content::ByteBuf(ref b) => Unexpected::Bytes(b.clone()),
            // Other match arms omitted for brevity
        }
    }
}

#[test]
fn test_unexpected_with_bytebuf() {
    let byte_buf_content = Content::ByteBuf(vec![1, 2, 3, 4, 5]);
    let result = byte_buf_content.unexpected();
    
    match result {
        Unexpected::Bytes(ref b) => {
            assert_eq!(b, &vec![1, 2, 3, 4, 5]);
        },
        _ => panic!("Expected Unexpected::Bytes variant"),
    }
}

#[test]
fn test_unexpected_with_empty_bytebuf() {
    let byte_buf_content = Content::ByteBuf(vec![]);
    let result = byte_buf_content.unexpected();
    
    match result {
        Unexpected::Bytes(ref b) => {
            assert_eq!(b, &vec![]);
        },
        _ => panic!("Expected Unexpected::Bytes variant"),
    }
}

