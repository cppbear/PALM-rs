// Answer 0

#[test]
fn test_new_creates_empty_bytes() {
    let byte_str = ByteStr::new();
    assert_eq!(byte_str.bytes.len(), 0);
}

#[test]
fn test_new_creates_different_instances() {
    let byte_str1 = ByteStr::new();
    let byte_str2 = ByteStr::new();
    assert_ne!(byte_str1, byte_str2);
}

