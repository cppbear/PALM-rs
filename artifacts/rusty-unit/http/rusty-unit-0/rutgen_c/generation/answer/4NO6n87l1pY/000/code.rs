// Answer 0

#[test]
fn test_from_utf8_valid_utf8() {
    let valid_bytes = Bytes::from_static(b"valid utf8 string");
    let result = ByteStr::from_utf8(valid_bytes);
    assert!(result.is_ok());
    if let Ok(byte_str) = result {
        assert_eq!(byte_str.bytes, Bytes::from_static(b"valid utf8 string"));
    }
}

#[test]
fn test_from_utf8_invalid_utf8() {
    let invalid_bytes = Bytes::from_static(b"\xFF\xFE\xFD");
    let result = ByteStr::from_utf8(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_utf8_empty_string() {
    let empty_bytes = Bytes::from_static(b"");
    let result = ByteStr::from_utf8(empty_bytes);
    assert!(result.is_ok());
    if let Ok(byte_str) = result {
        assert_eq!(byte_str.bytes, Bytes::from_static(b""));
    }
}

