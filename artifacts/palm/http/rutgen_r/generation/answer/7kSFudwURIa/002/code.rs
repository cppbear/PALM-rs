// Answer 0

#[test]
fn test_path_empty_uri() {
    struct EmptyUri;

    impl EmptyUri {
        fn has_path(&self) -> bool {
            false
        }

        fn path_and_query(&self) -> &str {
            ""
        }
    }

    let uri = EmptyUri;
    assert_eq!(uri.path(), "");
}

#[test]
fn test_path_uri_with_no_path() {
    struct NoPathUri;

    impl NoPathUri {
        fn has_path(&self) -> bool {
            false
        }

        fn path_and_query(&self) -> &str {
            ""
        }
    }

    let uri = NoPathUri;
    assert_eq!(uri.path(), "");
}

#[test]
fn test_path_uri_with_path_query() {
    struct NoPathQueryUri;

    impl NoPathQueryUri {
        fn has_path(&self) -> bool {
            false
        }
        
        fn path_and_query(&self) -> &str {
            ""
        }
    }

    let uri = NoPathQueryUri;
    assert_eq!(uri.path(), "");
}

#[test]
fn test_path_star_uri() {
    struct StarUri;

    impl StarUri {
        fn has_path(&self) -> bool {
            true
        }

        fn path_and_query(&self) -> &str {
            "*"
        }
    }

    let uri = StarUri;
    assert_eq!(uri.path(), "*");
}

