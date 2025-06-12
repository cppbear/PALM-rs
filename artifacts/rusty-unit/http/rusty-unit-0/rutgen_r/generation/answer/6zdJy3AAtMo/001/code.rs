// Answer 0

#[test]
fn test_into_parts_with_path_and_query() {
    let uri: http::Uri = "/foo?bar=baz".parse().unwrap();
    let parts = uri.into_parts();
    assert_eq!(parts.path_and_query.unwrap(), "/foo?bar=baz");
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

#[test]
fn test_into_parts_with_only_path() {
    let uri: http::Uri = "/foo".parse().unwrap();
    let parts = uri.into_parts();
    assert_eq!(parts.path_and_query.unwrap(), "/foo");
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

#[test]
fn test_into_parts_with_empty_path() {
    let uri: http::Uri = "/".parse().unwrap();
    let parts = uri.into_parts();
    assert_eq!(parts.path_and_query.unwrap(), "/");
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

#[test]
fn test_into_parts_with_no_path() {
    let uri: http::Uri = "http://example.com".parse().unwrap();
    let parts = uri.into_parts();
    assert!(parts.path_and_query.is_none());
    assert_eq!(parts.scheme.unwrap().as_str(), "http");
    assert_eq!(parts.authority.unwrap().as_str(), "example.com");
} 

#[should_panic]
fn test_into_parts_with_invalid_uri() {
    let uri: http::Uri = "invalid_uri".parse().unwrap_err();
    // Calling into_parts on an invalid URI should not succeed
    let _parts = uri.into_parts(); 
}

