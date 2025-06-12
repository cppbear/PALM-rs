// Answer 0

#[test]
fn test_unexpected_with_bytes() {
    let content = Content::Bytes(vec![1, 2, 3, 4, 5]);
    let unexpected = content.unexpected();
    match unexpected {
        Unexpected::Bytes(b) => assert_eq!(b, &[1, 2, 3, 4, 5]),
        _ => panic!("Expected Unexpected::Bytes variant"),
    }
}

#[test]
fn test_unexpected_with_empty_bytes() {
    let content = Content::Bytes(vec![]);
    let unexpected = content.unexpected();
    match unexpected {
        Unexpected::Bytes(b) => assert_eq!(b, &[]),
        _ => panic!("Expected Unexpected::Bytes variant"),
    }
}

