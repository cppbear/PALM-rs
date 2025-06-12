// Answer 0

#[test]
fn test_is_empty_when_empty() {
    let bytes = Bytes::new();
    assert!(bytes.is_empty());
}

#[test]
fn test_is_empty_when_not_empty() {
    let bytes = Bytes::copy_from_slice(&[1, 2, 3]);
    assert!(!bytes.is_empty());
}

