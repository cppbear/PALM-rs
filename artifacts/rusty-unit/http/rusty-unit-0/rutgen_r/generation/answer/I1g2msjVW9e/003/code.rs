// Answer 0

#[test]
fn test_create_authority_valid_case() {
    struct Bytes(Vec<u8>);

    struct Authority {
        data: ByteStr,
    }

    struct ByteStr(Vec<u8>);

    impl ByteStr {
        unsafe fn from_utf8_unchecked(bytes: Bytes) -> Self {
            ByteStr(bytes.0)
        }
    }

    impl Authority {
        fn parse_non_empty(s: &[u8]) -> Result<usize, String> {
            if s.is_empty() {
                Err("Empty input".to_string())
            } else {
                Ok(s.len()) // For the sake of this test, assume valid parsing returns the length
            }
        }
    }

    let b = b"example.com";
    let f = |b: &[u8]| Bytes(b.to_vec());

    let result = create_authority(b, f);
    assert!(result.is_ok());

    if let Ok(authority) = result {
        assert_eq!(authority.data.0, b"example.com");
    }
}

#[test]
#[should_panic]
fn test_create_authority_empty_input() {
    struct Bytes(Vec<u8>);

    struct Authority {
        data: ByteStr,
    }

    struct ByteStr(Vec<u8>);

    impl ByteStr {
        unsafe fn from_utf8_unchecked(bytes: Bytes) -> Self {
            ByteStr(bytes.0)
        }
    }

    impl Authority {
        fn parse_non_empty(s: &[u8]) -> Result<usize, String> {
            if s.is_empty() {
                Err("Empty input".to_string())
            } else {
                Ok(s.len())
            }
        }
    }

    let b = b"";
    let f = |b: &[u8]| Bytes(b.to_vec());

    create_authority(b, f).unwrap();
}

#[test]
fn test_create_authority_bounds_case() {
    struct Bytes(Vec<u8>);

    struct Authority {
        data: ByteStr,
    }

    struct ByteStr(Vec<u8>);

    impl ByteStr {
        unsafe fn from_utf8_unchecked(bytes: Bytes) -> Self {
            ByteStr(bytes.0)
        }
    }

    impl Authority {
        fn parse_non_empty(s: &[u8]) -> Result<usize, String> {
            if s.is_empty() {
                Err("Empty input".to_string())
            } else {
                Ok(s.len())
            }
        }
    }

    let b = b"localhost:8080";
    let f = |b: &[u8]| Bytes(b.to_vec());

    let result = create_authority(b, f);
    assert!(result.is_ok());

    if let Ok(authority) = result {
        assert_eq!(authority.data.0, b"localhost:8080");
    }
}

