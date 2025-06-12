// Answer 0

#[test]
fn test_path_when_query_is_none_and_data_is_empty() {
    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }
    
    impl PathAndQuery {
        fn new_empty() -> Self {
            TestPathAndQuery {
                data: ByteStr::from_static(""),
                query: NONE,
            }
        }
    }

    let path_and_query = TestPathAndQuery::new_empty();
    assert_eq!(path_and_query.path(), "/");
}

#[test]
fn test_path_when_query_is_none_and_data_is_non_empty() {
    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }
    
    impl PathAndQuery {
        fn new(path: &str) -> Self {
            TestPathAndQuery {
                data: ByteStr::from_static(path),
                query: NONE,
            }
        }
    }

    let path_and_query = TestPathAndQuery::new("/hello/world");
    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
fn test_path_when_query_is_none_and_data_is_slash() {
    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    impl PathAndQuery {
        fn new(path: &str) -> Self {
            TestPathAndQuery {
                data: ByteStr::from_static(path),
                query: NONE,
            }
        }
    }

    let path_and_query = TestPathAndQuery::new("/");
    assert_eq!(path_and_query.path(), "/");
}

