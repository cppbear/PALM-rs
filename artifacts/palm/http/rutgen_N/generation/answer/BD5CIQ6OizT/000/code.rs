// Answer 0

#[test]
fn test_from_bytes_valid_input() {
    struct HdrName<'a>(&'a [u8]);

    let header_bytes = b"Valid-Header-Name";
    let result = from_bytes(header_bytes, |hdr| {
        assert_eq!(hdr.0, b"Valid-Header-Name");
        hdr
    });

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_from_bytes_invalid_input() {
    struct HdrName<'a>(&'a [u8]);

    let header_bytes = b"Invalid\x00Header"; // invalid character
    let result = from_bytes(header_bytes, |hdr| hdr);

    assert!(result.is_err());
}

