// Answer 0

#[test]
fn test_unexpected_with_bytebuf() {
    use std::vec::Vec;
    let content = Content::ByteBuf(vec![1, 2, 3, 4]);

    match content.unexpected() {
        Unexpected::Bytes(b) => {
            assert_eq!(b, &[1, 2, 3, 4]);
        }
        _ => {
            panic!("Expected Unexpected::Bytes variant");
        }
    }
}

#[test]
fn test_unexpected_with_empty_bytebuf() {
    use std::vec::Vec;
    let content = Content::ByteBuf(vec![]);

    match content.unexpected() {
        Unexpected::Bytes(b) => {
            assert_eq!(b, &[]);
        }
        _ => {
            panic!("Expected Unexpected::Bytes variant");
        }
    }
}

