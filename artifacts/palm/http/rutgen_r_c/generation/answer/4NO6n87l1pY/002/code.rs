// Answer 0

#[test]
fn test_from_utf8_valid_utf8() {
    use bytes::Bytes;

    let valid_utf8_bytes = Bytes::from_static(b"Hello, World!");
    let result = ByteStr::from_utf8(valid_utf8_bytes);

    assert!(result.is_ok());
    if let Ok(byte_str) = result {
        assert_eq!(byte_str, ByteStr { bytes: valid_utf8_bytes });
    }
}

#[test]
fn test_from_utf8_empty_string() {
    use bytes::Bytes;

    let empty_bytes = Bytes::from_static(b"");
    let result = ByteStr::from_utf8(empty_bytes);

    assert!(result.is_ok());
    if let Ok(byte_str) = result {
        assert_eq!(byte_str, ByteStr { bytes: empty_bytes });
    }
}

#[test]
fn test_from_utf8_valid_unicode() {
    use bytes::Bytes;

    let unicode_bytes = Bytes::from_static("こんにちは".as_bytes());
    let result = ByteStr::from_utf8(unicode_bytes);

    assert!(result.is_ok());
    if let Ok(byte_str) = result {
        assert_eq!(byte_str, ByteStr { bytes: unicode_bytes });
    }
}

