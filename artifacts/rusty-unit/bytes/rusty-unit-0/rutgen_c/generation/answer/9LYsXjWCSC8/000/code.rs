// Answer 0

#[test]
fn test_as_slice_non_empty() {
    let bytes: Bytes = Bytes::from_static(b"hello");
    let slice = bytes.as_slice();
    assert_eq!(slice, b"hello");
}

#[test]
fn test_as_slice_empty() {
    let bytes: Bytes = Bytes::new();
    let slice = bytes.as_slice();
    assert!(slice.is_empty());
}

#[test]
fn test_as_slice_with_length() {
    let bytes: Bytes = Bytes::from_static(b"123456");
    let slice = bytes.as_slice();
    assert_eq!(slice.len(), 6);
}

#[test]
fn test_as_slice_boundary() {
    let bytes: Bytes = Bytes::from_static(b"abcdef");
    let slice = bytes.as_slice();
    assert_eq!(slice[0], b'a');
    assert_eq!(slice[5], b'f');
}

