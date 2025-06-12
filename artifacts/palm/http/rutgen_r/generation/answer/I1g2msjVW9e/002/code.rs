// Answer 0

#[test]
fn test_create_authority_invalid_uri_char() {
    struct TestBytes(Vec<u8>);

    impl AsRef<[u8]> for TestBytes {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    let invalid_uri = TestBytes(b"valid_authority_invalid_char*");
    let result = http::create_authority(invalid_uri, |b| b.into());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), http::ErrorKind::InvalidUriChar);
}

#[test]
fn test_create_authority_non_utf8() {
    struct TestBytes(Vec<u8>);

    impl AsRef<[u8]> for TestBytes {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    let non_utf8_uri = TestBytes(b"\x80\x81invalid");
    let result = http::create_authority(non_utf8_uri, |b| b.into());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), http::ErrorKind::InvalidUriChar);
}

#[test]
fn test_create_authority_too_long() {
    struct TestBytes(Vec<u8>);

    impl AsRef<[u8]> for TestBytes {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    let long_uri = TestBytes(b"verylongauthoritystringthatexceedsthelimit");
    let result = http::create_authority(long_uri, |b| b.into());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), http::ErrorKind::InvalidUriChar);
}

