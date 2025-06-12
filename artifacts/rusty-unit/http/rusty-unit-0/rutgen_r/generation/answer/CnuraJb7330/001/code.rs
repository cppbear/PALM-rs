// Answer 0

#[test]
fn test_scheme_none_for_relative_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.scheme().is_none());
}

#[test]
fn test_scheme_none_for_empty_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "".parse().unwrap();
    assert!(uri.scheme().is_none());
}

#[test]
fn test_scheme_none_for_fragment_only_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "#fragid1".parse().unwrap();
    assert!(uri.scheme().is_none());
}

#[test]
fn test_scheme_none_for_query_only_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "?key=value".parse().unwrap();
    assert!(uri.scheme().is_none());
}

