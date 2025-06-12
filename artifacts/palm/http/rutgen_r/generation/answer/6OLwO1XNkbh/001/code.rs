// Answer 0

#[test]
fn test_slash() {
    struct PathAndQuery {
        data: ByteStr,
        query: Option<String>,
    }

    impl PathAndQuery {
        fn slash() -> Self {
            PathAndQuery {
                data: ByteStr::from_static("/"),
                query: None,
            }
        }
    }

    struct ByteStr {
        bytes: &'static str,
    }

    impl ByteStr {
        fn from_static(data: &'static str) -> Self {
            ByteStr { bytes: data }
        }
    }

    const NONE: Option<String> = None;

    let result = PathAndQuery::slash();
    
    // Validate the data
    assert_eq!(result.data.bytes, "/");
    // Validate the query
    assert_eq!(result.query, NONE);
}

