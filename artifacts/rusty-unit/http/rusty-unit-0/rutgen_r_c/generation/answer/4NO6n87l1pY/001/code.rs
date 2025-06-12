// Answer 0

#[test]
fn test_from_utf8_invalid_utf8_bytes() {
    use bytes::Bytes;

    // Create bytes that are not valid UTF-8. For example, use bytes that contain values from 0x80 to 0xFF.
    let invalid_bytes = Bytes::from_static(&[0xFF, 0xFE, 0xFD]);

    // Ensure the function returns an Err when given invalid UTF-8 bytes.
    let result = ByteStr::from_utf8(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_utf8_valid_utf8_bytes() {
    use bytes::Bytes;

    // Create valid UTF-8 bytes, like "hello".
    let valid_bytes = Bytes::from_static(b"hello");

    // Ensure the function returns Ok when given valid UTF-8 bytes.
    let result = ByteStr::from_utf8(valid_bytes);
    assert!(result.is_ok());

    // If it succeeds, we should be able to unwrap and check the contained ByteStr
    let byte_str = result.unwrap();
    assert_eq!(byte_str.bytes, valid_bytes);
}

#[test]
fn test_from_utf8_empty_bytes() {
    use bytes::Bytes;

    // Create an empty byte sequence.
    let empty_bytes = Bytes::from_static(&[]);

    // Ensure the function successfully returns Ok for empty bytes.
    let result = ByteStr::from_utf8(empty_bytes);
    assert!(result.is_ok());

    // Check the contents of the returned ByteStr
    let byte_str = result.unwrap();
    assert_eq!(byte_str.bytes.len(), 0);
}

