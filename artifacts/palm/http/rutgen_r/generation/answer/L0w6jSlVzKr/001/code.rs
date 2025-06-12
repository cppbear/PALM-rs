// Answer 0

#[test]
fn test_absolute_uri_with_query() {
    use http::Uri;
    let uri: Uri = "http://example.org/hello/world?key=value".parse().unwrap();
    assert_eq!(uri.query(), Some("key=value"));
}

#[test]
fn test_absolute_uri_with_multiple_queries() {
    use http::Uri;
    let uri: Uri = "http://example.org/hello/world?key=value&key2=value2#fragid1".parse().unwrap();
    assert_eq!(uri.query(), Some("key=value&key2=value2"));
}

#[test]
fn test_relative_uri_with_query() {
    use http::Uri;
    let uri: Uri = "/hello/world?key=value&foo=bar".parse().unwrap();
    assert_eq!(uri.query(), Some("key=value&foo=bar"));
}

#[test]
fn test_relative_uri_without_query() {
    use http::Uri;
    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.query().is_none());
}

#[test]
fn test_uri_with_only_query() {
    use http::Uri;
    let uri: Uri = "http://example.org/?key=value".parse().unwrap();
    assert_eq!(uri.query(), Some("key=value"));
}

#[test]
fn test_uri_with_empty_query() {
    use http::Uri;
    let uri: Uri = "http://example.org/hello/world?".parse().unwrap();
    assert_eq!(uri.query(), Some(""));
}

