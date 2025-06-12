// Answer 0

#[test]
fn test_from_static_valid_uri() {
    let uri = Uri::from_static("http://example.com/foo");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
#[should_panic]
fn test_from_static_invalid_uri() {
    let _uri = Uri::from_static("invalid_uri");
}

#[test]
fn test_from_static_empty_uri() {
    let uri = Uri::from_static("http://example.com/");
    assert_eq!(uri.path(), "/");
}

#[test]
fn test_from_static_uri_with_query() {
    let uri = Uri::from_static("http://example.com/foo?bar=baz");
    assert_eq!(uri.query().unwrap(), "bar=baz");
}

