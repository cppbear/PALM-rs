// Answer 0

#[test]
fn test_from_shared_valid_uri_with_fragment_and_invalid_utf8() {
    use bytes::Bytes;

    struct TestBytes {
        bytes: Bytes,
    }

    impl TestBytes {
        fn new(data: &[u8]) -> Self {
            Self { bytes: Bytes::from(data) }
        }
    }

    let input_data = b"/path/to/resource#fragment\xFF"; // valid path, invalid UTF-8
    let bytes = TestBytes::new(input_data).bytes;

    match PathAndQuery::from_shared(bytes) {
        Ok(path_and_query) => {
            assert_eq!(path_and_query.query, u16::MAX); // query should be NONE
            assert!(path_and_query.data.bytes.len() == input_data.len() - 10); // check that truncation handled correctly
        },
        _ => panic!("Expected Ok but got Err"),
    }
}

#[test]
fn test_from_shared_valid_query_missing() {
    use bytes::Bytes;

    struct TestBytes {
        bytes: Bytes,
    }

    impl TestBytes {
        fn new(data: &[u8]) -> Self {
            Self { bytes: Bytes::from(data) }
        }
    }

    let input_data = b"/path/to/resource#fragment"; // no query present
    let bytes = TestBytes::new(input_data).bytes;

    match PathAndQuery::from_shared(bytes) {
        Ok(path_and_query) => {
            assert_eq!(path_and_query.query, u16::MAX); // query should be NONE
            assert!(path_and_query.data.bytes.len() == input_data.len() - 10); // check length
        },
        _ => panic!("Expected Ok but got Err"),
    }
}

#[test]
fn test_from_shared_invalid_uri_character() {
    use bytes::Bytes;

    struct TestBytes {
        bytes: Bytes,
    }

    impl TestBytes {
        fn new(data: &[u8]) -> Self {
            Self { bytes: Bytes::from(data) }
        }
    }

    let input_data = b"/path/to/resou?ce#fragment\x81"; // invalid character (0x81)
    let bytes = TestBytes::new(input_data).bytes;

    match PathAndQuery::from_shared(bytes) {
        Err(InvalidUri(ErrorKind::InvalidUriChar)) => {},
        _ => panic!("Expected Err with InvalidUriChar but got Ok or a different error"),
    }
}

