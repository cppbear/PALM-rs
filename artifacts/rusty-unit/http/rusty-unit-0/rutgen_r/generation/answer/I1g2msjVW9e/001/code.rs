// Answer 0

#[test]
fn test_create_authority_invalid_empty() {
    let result = create_authority(b"", |b| Bytes::from(b));
    assert!(result.is_err());
}

#[test]
fn test_create_authority_invalid_non_utf8() {
    struct NonUtf8Bytes;
    impl AsRef<[u8]> for NonUtf8Bytes {
        fn as_ref(&self) -> &[u8] {
            b"\xFF\xFE\xFD" // Invalid UTF-8 sequence
        }
    }

    let result = create_authority(NonUtf8Bytes, |b| Bytes::from(b));
    assert!(result.is_err());
}

#[test]
fn test_create_authority_invalid_uri_char() {
    let input = b"invalid_uri:";
    let result = create_authority(input, |b| Bytes::from(b));
    assert!(result.is_err());
}

#[test]
fn test_create_authority_boundary_utf8() {
    let input = b"valid-uri";
    let result = create_authority(input, |b| Bytes::from(b));
    assert!(result.is_ok());
    let authority = result.unwrap();
    assert_eq!(authority.data, ByteStr::from_utf8_unchecked(Bytes::from(input)));
}

