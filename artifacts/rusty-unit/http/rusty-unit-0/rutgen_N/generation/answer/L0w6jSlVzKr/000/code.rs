// Answer 0

#[test]
fn test_query_absolute_uri() {
    struct Uri {
        path_and_query: String,
    }

    impl Uri {
        fn query(&self) -> Option<&str> {
            self.path_and_query.split('?').skip(1).next().and_then(|s| s.split('#').next())
        }
    }

    let uri = Uri {
        path_and_query: "http://example.org/hello/world?key=value".to_string(),
    };

    assert_eq!(uri.query(), Some("key=value"));
}

#[test]
fn test_query_relative_uri_with_query() {
    struct Uri {
        path_and_query: String,
    }

    impl Uri {
        fn query(&self) -> Option<&str> {
            self.path_and_query.split('?').skip(1).next().and_then(|s| s.split('#').next())
        }
    }

    let uri = Uri {
        path_and_query: "/hello/world?key=value&foo=bar".to_string(),
    };

    assert_eq!(uri.query(), Some("key=value&foo=bar"));
}

#[test]
fn test_query_relative_uri_without_query() {
    struct Uri {
        path_and_query: String,
    }

    impl Uri {
        fn query(&self) -> Option<&str> {
            self.path_and_query.split('?').skip(1).next().and_then(|s| s.split('#').next())
        }
    }

    let uri = Uri {
        path_and_query: "/hello/world".to_string(),
    };

    assert!(uri.query().is_none());
}

