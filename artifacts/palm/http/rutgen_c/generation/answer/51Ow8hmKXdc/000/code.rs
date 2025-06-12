// Answer 0

#[test]
fn test_from_utf8_unchecked_valid_utf8() {
    let valid_bytes = Bytes::from_static(b"valid");
    let byte_str = unsafe { ByteStr::from_utf8_unchecked(valid_bytes) };
    assert_eq!(byte_str.bytes, Bytes::from_static(b"valid"));
}

#[should_panic(expected = "ByteStr::from_utf8_unchecked() with invalid bytes")]
#[test]
fn test_from_utf8_unchecked_invalid_utf8() {
    let invalid_bytes = Bytes::from_static(b"\xFF\xFE");
    unsafe { ByteStr::from_utf8_unchecked(invalid_bytes) };
}

