// Answer 0

#[test]
#[should_panic]
fn test_from_utf8_unchecked_with_invalid_utf8() {
    use bytes::Bytes;
    
    let invalid_bytes = Bytes::from(vec![0, 159, 146, 150]); // Example of invalid UTF-8 bytes
    unsafe {
        ByteStr::from_utf8_unchecked(invalid_bytes);
    }
}

#[test]
fn test_from_utf8_unchecked_with_valid_utf8() {
    use bytes::Bytes;

    let valid_bytes = Bytes::from("Hello, World!".as_bytes()); // Valid UTF-8
    let byte_str = unsafe { ByteStr::from_utf8_unchecked(valid_bytes) };

    assert_eq!(byte_str.bytes, Bytes::from("Hello, World!".as_bytes()));
}

