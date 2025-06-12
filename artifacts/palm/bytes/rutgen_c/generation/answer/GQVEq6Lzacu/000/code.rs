// Answer 0

#[test]
fn test_clear_on_non_empty_bytes() {
    let mut buf = Bytes::copy_from_slice(&[1, 2, 3, 4, 5]);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_clear_on_empty_bytes() {
    let mut buf = Bytes::new();
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_clear_on_large_bytes() {
    let mut buf = Bytes::copy_from_slice(&[0u8; 1024]); // large buffer
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

