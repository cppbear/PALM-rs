// Answer 0

#[test]
fn test_authority_empty() {
    let authority = Authority::empty();
    assert_eq!(authority.data.bytes.len(), 0);
}

#[test]
fn test_authority_empty_byte_str() {
    let byte_str = ByteStr::new();
    assert_eq!(byte_str.bytes.len(), 0);
}

