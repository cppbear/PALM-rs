// Answer 0

#[test]
fn test_bytes_new_empty() {
    use bytes::Bytes;

    let b = Bytes::new();
    assert_eq!(&b[..], b"");
}

#[test]
fn test_bytes_new_non_empty() {
    use bytes::Bytes;

    let b = Bytes::from_static(b"hello");
    assert_eq!(&b[..], b"hello");
}

