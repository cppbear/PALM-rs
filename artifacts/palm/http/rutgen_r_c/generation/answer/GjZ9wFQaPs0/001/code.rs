// Answer 0

#[test]
fn test_new_byte_str() {
    let byte_str = ByteStr::new();
    assert_eq!(byte_str, ByteStr { bytes: Bytes::new() });
}

#[test]
fn test_byte_str_empty() {
    let byte_str = ByteStr::new();
    assert_eq!(byte_str.bytes.len(), 0);
}

