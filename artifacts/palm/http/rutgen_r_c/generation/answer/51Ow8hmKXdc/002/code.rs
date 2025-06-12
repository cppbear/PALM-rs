// Answer 0

#[test]
fn test_from_utf8_unchecked_valid_utf8() {
    let valid_utf8_bytes = Bytes::from_static(b"valid utf8 string");
    let byte_str = unsafe { ByteStr::from_utf8_unchecked(valid_utf8_bytes.clone()) };
    assert_eq!(byte_str.bytes, valid_utf8_bytes);
}

#[test]
#[should_panic]
fn test_from_utf8_unchecked_invalid_utf8() {
    let invalid_utf8_bytes = Bytes::from_static(b"\xff"); // Invalid UTF-8 sequence
    unsafe { ByteStr::from_utf8_unchecked(invalid_utf8_bytes) }; // This should panic in debug mode
}

