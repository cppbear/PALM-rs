// Answer 0

#[test]
fn test_star() {
    struct PathAndQuery {
        data: ByteStr,
        query: Option<String>, // Assuming NONE is of type Option<String>
    }

    impl PathAndQuery {
        fn star() -> Self {
            PathAndQuery {
                data: ByteStr::from_static("*"),
                query: None,
            }
        }
    }

    struct ByteStr {
        bytes: &'static str,
    }

    impl ByteStr {
        fn from_static(bytes: &'static str) -> Self {
            ByteStr { bytes }
        }
    }

    let path_query = PathAndQuery::star();
    assert_eq!(path_query.data.bytes, "*");
    assert!(path_query.query.is_none());
}

