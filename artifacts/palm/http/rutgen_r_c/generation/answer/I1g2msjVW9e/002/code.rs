// Answer 0

#[test]
fn test_create_authority_invalid_uri_char() {
    struct BytesWrapper {
        bytes: Bytes,
    }

    impl AsRef<[u8]> for BytesWrapper {
        fn as_ref(&self) -> &[u8] {
            &self.bytes
        }
    }

    fn create_bytes_wrapper(input: &[u8]) -> BytesWrapper {
        BytesWrapper {
            bytes: Bytes::copy_from_slice(input),
        }
    }

    let input: &[u8] = b"abc:def"; // This should cause an invalid URI char error
    let wrapper = create_bytes_wrapper(input);

    let result = create_authority(wrapper, |b| Bytes::copy_from_slice(b.as_ref()));

    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidUriChar)));
}

#[test]
fn test_create_authority_invalid_uri_char_different_input() {
    struct BytesWrapper {
        bytes: Bytes,
    }

    impl AsRef<[u8]> for BytesWrapper {
        fn as_ref(&self) -> &[u8] {
            &self.bytes
        }
    }

    fn create_bytes_wrapper(input: &[u8]) -> BytesWrapper {
        BytesWrapper {
            bytes: Bytes::copy_from_slice(input),
        }
    }

    let input: &[u8] = b"//invalid"; // This should also trigger an invalid URI char error
    let wrapper = create_bytes_wrapper(input);

    let result = create_authority(wrapper, |b| Bytes::copy_from_slice(b.as_ref()));

    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidUriChar)));
}

