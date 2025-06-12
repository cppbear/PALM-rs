// Answer 0

#[test]
fn test_path_and_query_with_scheme_and_authority() {
    struct Scheme {
        inner: Option<String>,
    }

    struct Authority {
        data: String,
    }

    struct PathAndQuery {
        path: String,
        query: Option<String>,
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    let uri = Uri {
        scheme: Scheme { inner: Some("http".to_string()) },
        authority: Authority { data: "example.com".to_string() },
        path_and_query: PathAndQuery {
            path: "/some/path".to_string(),
            query: None,
        },
    };

    let result = uri.path_and_query();
    assert!(result.is_some());
    assert_eq!(result.unwrap().path, "/some/path");
}

#[test]
fn test_path_and_query_without_scheme() {
    struct Scheme {
        inner: Option<String>,
    }

    struct Authority {
        data: String,
    }

    struct PathAndQuery {
        path: String,
        query: Option<String>,
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    let uri = Uri {
        scheme: Scheme { inner: None },
        authority: Authority { data: "example.com".to_string() },
        path_and_query: PathAndQuery {
            path: "/some/path".to_string(),
            query: None,
        },
    };

    let result = uri.path_and_query();
    assert!(result.is_none());
}

#[test]
fn test_path_and_query_without_authority() {
    struct Scheme {
        inner: Option<String>,
    }

    struct Authority {
        data: String,
    }

    struct PathAndQuery {
        path: String,
        query: Option<String>,
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    let uri = Uri {
        scheme: Scheme { inner: Some("http".to_string()) },
        authority: Authority { data: "".to_string() },
        path_and_query: PathAndQuery {
            path: "/some/path".to_string(),
            query: None,
        },
    };

    let result = uri.path_and_query();
    assert!(result.is_none());
}

