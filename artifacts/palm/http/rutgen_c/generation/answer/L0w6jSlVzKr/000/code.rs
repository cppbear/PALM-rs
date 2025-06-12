// Answer 0

#[test]
fn test_query_with_absolute_uri() {
    let uri: Uri = Uri::from_static("http://example.org/hello/world?key=value");
    assert_eq!(uri.query(), Some("key=value"));
}

#[test]
fn test_query_with_relative_uri() {
    let uri: Uri = Uri::from_static("/hello/world?key=value&foo=bar");
    assert_eq!(uri.query(), Some("key=value&foo=bar"));
}

#[test]
fn test_query_without_query_string() {
    let uri: Uri = Uri::from_static("/hello/world");
    assert!(uri.query().is_none());
}

#[test]
fn test_query_with_empty_query() {
    let uri: Uri = Uri::from_static("/hello/world?");
    assert_eq!(uri.query(), Some(""));
}

