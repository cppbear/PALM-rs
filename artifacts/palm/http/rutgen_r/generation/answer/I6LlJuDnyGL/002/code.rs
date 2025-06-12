// Answer 0

#[test]
fn test_valid_uri() {
    let uri = Uri::from_static("http://example.com/foo");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
fn test_valid_uri_with_port() {
    let uri = Uri::from_static("http://example.com:8080/foo");
    assert_eq!(uri.host().unwrap(), "example.com:8080");
    assert_eq!(uri.path(), "/foo");
}

#[test]
fn test_valid_uri_with_query() {
    let uri = Uri::from_static("http://example.com/foo?bar=baz");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
    assert_eq!(uri.query().unwrap(), "bar=baz");
}

#[test]
fn test_valid_uri_with_fragment() {
    let uri = Uri::from_static("http://example.com/foo#section");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
    assert_eq!(uri.fragment().unwrap(), "section");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_invalid_uri_empty_string() {
    Uri::from_static("");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_invalid_uri_no_scheme() {
    Uri::from_static("example.com/foo");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_invalid_uri_invalid_chars() {
    Uri::from_static("http://example.com/foo#@");
}

