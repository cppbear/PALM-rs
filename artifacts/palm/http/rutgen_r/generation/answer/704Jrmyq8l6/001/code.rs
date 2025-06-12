// Answer 0

#[derive(Default)]
struct Scheme {
    inner: Option<String>,
}

#[derive(Default)]
struct PathAndQuery {
    data: String,
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
fn test_has_path_with_empty_data_and_none_scheme() {
    let uri = Uri {
        path_and_query: PathAndQuery::default(), // Empty path_and_query.data
        scheme: Scheme { inner: None }, // None scheme.inner
    };
    assert!(!uri.has_path()); // Expected output is false
}

#[test]
fn test_has_path_with_empty_data_and_some_scheme() {
    let uri = Uri {
        path_and_query: PathAndQuery::default(), // Empty path_and_query.data
        scheme: Scheme { inner: Some(String::from("http")) }, // Some scheme.inner
    };
    assert!(uri.has_path()); // Expected output is true
}

#[test]
fn test_has_path_with_non_empty_data_and_none_scheme() {
    let uri = Uri {
        path_and_query: PathAndQuery { data: String::from("/") }, // Non-empty path_and_query.data
        scheme: Scheme { inner: None }, // None scheme.inner
    };
    assert!(uri.has_path()); // Expected output is true
}

#[test]
fn test_has_path_with_non_empty_data_and_some_scheme() {
    let uri = Uri {
        path_and_query: PathAndQuery { data: String::from("/") }, // Non-empty path_and_query.data
        scheme: Scheme { inner: Some(String::from("http")) }, // Some scheme.inner
    };
    assert!(uri.has_path()); // Expected output is true
}

