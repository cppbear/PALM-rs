// Answer 0

#[test]
fn test_create_authority_valid_input() {
    struct BytesProvider;
    impl BytesProvider {
        fn get_bytes() -> Bytes {
            Bytes::from_static(b"example.com")
        }
    }

    let result = create_authority(b"example.com", |b| Bytes::copy_from_slice(b));
    assert!(result.is_ok());
}

#[test]
fn test_create_authority_invalid_uri_char() {
    struct BytesProvider;
    impl BytesProvider {
        fn get_bytes() -> Bytes {
            Bytes::from_static(b"invalid#char")
        }
    }

    let result = create_authority(b"invalid#char", |b| Bytes::copy_from_slice(b));
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidUriChar);
    }
}

#[test]
fn test_create_authority_empty_string() {
    struct BytesProvider;
    impl BytesProvider {
        fn get_bytes() -> Bytes {
            Bytes::from_static(b"")
        }
    }

    let result = create_authority(b"", |b| Bytes::copy_from_slice(b));
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::Empty);
    }
}

#[test]
fn test_create_authority_valid_utf8() {
    struct BytesProvider;
    impl BytesProvider {
        fn get_bytes() -> Bytes {
            Bytes::from_static(b"valid-utf8.com")
        }
    }

    let result = create_authority(b"valid-utf8.com", |b| Bytes::copy_from_slice(b));
    assert!(result.is_ok());
}

