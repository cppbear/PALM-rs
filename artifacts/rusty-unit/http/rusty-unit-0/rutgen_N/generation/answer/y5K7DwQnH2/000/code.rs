// Answer 0

#[test]
fn test_empty_path_and_query() {
    struct PathAndQuery {
        data: ByteStr,
        query: Option<String>,
    }

    struct ByteStr {
        bytes: Vec<u8>,
    }

    impl ByteStr {
        fn new() -> Self {
            ByteStr { bytes: Vec::new() }
        }
    }

    const NONE: Option<String> = None;

    fn empty() -> PathAndQuery {
        PathAndQuery {
            data: ByteStr::new(),
            query: NONE,
        }
    }

    let path_and_query = empty();
    assert_eq!(path_and_query.data.bytes.len(), 0);
    assert!(path_and_query.query.is_none());
}

