// Answer 0

#[test]
fn test_len_empty() {
    let bytes = Bytes::new();
    assert_eq!(bytes.len(), 0);
}

#[test]
fn test_len_non_empty() {
    let bytes = Bytes::from_static(b"hello");
    assert_eq!(bytes.len(), 5);
}

#[test]
fn test_len_large_array() {
    let bytes = Bytes::from_static(b"hello, world!");
    assert_eq!(bytes.len(), 13);
}

