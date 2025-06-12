// Answer 0

#[test]
fn test_from_utf8_valid_utf8() {
    use http::{from_utf8, ByteStr};
    use bytes::Bytes;

    // Valid UTF-8 input
    let valid_utf8_bytes = Bytes::from("Hello, world!");
    let result = from_utf8(valid_utf8_bytes);
    assert!(result.is_ok());

    if let Ok(byte_str) = result {
        assert_eq!(byte_str.bytes, Bytes::from("Hello, world!"));
    }
}

#[test]
fn test_from_utf8_valid_utf8_with_special_chars() {
    use http::{from_utf8, ByteStr};
    use bytes::Bytes;

    // Valid UTF-8 input with special characters
    let valid_utf8_bytes = Bytes::from("你好，世界！");
    let result = from_utf8(valid_utf8_bytes);
    assert!(result.is_ok());

    if let Ok(byte_str) = result {
        assert_eq!(byte_str.bytes, Bytes::from("你好，世界！"));
    }
}

#[test]
fn test_from_utf8_empty_string() {
    use http::{from_utf8, ByteStr};
    use bytes::Bytes;

    // Valid UTF-8 input as an empty string
    let valid_utf8_bytes = Bytes::from("");
    let result = from_utf8(valid_utf8_bytes);
    assert!(result.is_ok());

    if let Ok(byte_str) = result {
        assert_eq!(byte_str.bytes, Bytes::from(""));
    }
}

