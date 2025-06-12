// Answer 0

#[derive(Default)]
struct PathAndQuery {
    data: String,
}

#[derive(Default)]
struct Scheme {
    inner: Option<String>,
}

struct Uri {
    path_and_query: PathAndQuery,
    scheme: Scheme,
}

impl Uri {
    fn has_path(&self) -> bool {
        !self.path_and_query.data.is_empty() || !self.scheme.inner.is_none()
    }
}

#[test]
fn test_has_path_with_non_empty_path_and_query() {
    let uri = Uri {
        path_and_query: PathAndQuery {
            data: String::from("/example"),
        },
        scheme: Scheme::default(),
    };
    assert!(uri.has_path());
}

#[test]
fn test_has_path_with_non_empty_scheme() {
    let uri = Uri {
        path_and_query: PathAndQuery::default(),
        scheme: Scheme {
            inner: Some(String::from("http")),
        },
    };
    assert!(uri.has_path());
}

#[test]
fn test_has_path_with_both_non_empty() {
    let uri = Uri {
        path_and_query: PathAndQuery {
            data: String::from("/example"),
        },
        scheme: Scheme {
            inner: Some(String::from("http")),
        },
    };
    assert!(uri.has_path());
}

#[test]
fn test_has_path_with_empty_path_and_query_and_empty_scheme() {
    let uri = Uri {
        path_and_query: PathAndQuery::default(),
        scheme: Scheme::default(),
    };
    assert!(!uri.has_path());
}

