// Answer 0

#[test]
fn test_query_none() {
    struct TestBytes {
        bytes: Bytes,
    }

    impl TestBytes {
        fn new(data: &[u8]) -> Self {
            Self { bytes: Bytes::from(data) }
        }
    }

    let empty_query = PathAndQuery {
        data: ByteStr::new(),
        query: NONE,
    };
    assert_eq!(empty_query.query(), None);

    let path_without_query = PathAndQuery {
        data: ByteStr::new(),
        query: NONE,
    };
    assert_eq!(path_without_query.query(), None);

    let path_with_only_fragment = PathAndQuery {
        data: ByteStr::from_static("/path/to/resource#fragment"),
        query: NONE,
    };
    assert_eq!(path_with_only_fragment.query(), None);

    let path_with_utf8_data = PathAndQuery {
        data: ByteStr::from_static("/path/to/resource"),
        query: NONE,
    };
    assert_eq!(path_with_utf8_data.query(), None);
}

