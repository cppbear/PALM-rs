// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    let result = HdrName::from_bytes(b"accept", |header| {
        assert_eq!(header.inner, Repr::Standard(StandardHeader::Accept));
    });
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_valid_custom_header() {
    let result = HdrName::from_bytes(b"x-custom-header", |header| {
        // Assuming the custom header is parsed correctly
        if let Repr::Custom(custom) = header.inner {
            assert_eq!(custom.buf, b"x-custom-header");
            assert!(custom.lower);
        } else {
            panic!("Expected custom header.");
        }
    });
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_empty_header() {
    let result = HdrName::from_bytes(b"", |header| {});
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_overflow_header() {
    let long_header = vec![b'a'; SCRATCH_BUF_OVERFLOW];
    let result = HdrName::from_bytes(&long_header, |header| {});
    assert!(result.is_ok());
} 

#[test]
fn test_from_bytes_invalid_header_with_null() {
    let result = HdrName::from_bytes(b"invalid\x00header", |header| {});
    assert!(result.is_err());
}

