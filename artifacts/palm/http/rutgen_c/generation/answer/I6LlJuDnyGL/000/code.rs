// Answer 0

#[test]
fn test_from_static_valid_uri() {
    let uri = Uri::from_static("http://example.com/foo");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_invalid_uri_empty() {
    Uri::from_static("");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_invalid_uri_too_long() {
    let uri_string = "http://example.com/".repeat(100); // Exceeding MAX_LEN
    Uri::from_static(&uri_string);
}

#[test]
fn test_from_static_valid_uri_with_query() {
    let uri = Uri::from_static("http://example.com/foo?query=true");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
    assert_eq!(uri.query().unwrap(), "query=true");
}

#[test]
fn test_from_static_uri_with_slash() {
    let uri = Uri::from_static("/");
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
    assert_eq!(uri.path(), "/");
}

#[test]
fn test_from_static_uri_with_star() {
    let uri = Uri::from_static("*");
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
    assert_eq!(uri.path(), "*");
}

