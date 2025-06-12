// Answer 0

#[test]
fn test_into_parts_with_path_only() {
    use http::uri::{Uri, Parts};
    let uri: Uri = "/foo".parse().unwrap();
    let parts = uri.into_parts();
    
    assert_eq!(parts.path_and_query.unwrap(), "/foo");
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

#[test]
fn test_into_parts_with_empty_path() {
    use http::uri::{Uri, Parts};
    let uri: Uri = "/".parse().unwrap();
    let parts = uri.into_parts();
    
    assert_eq!(parts.path_and_query.unwrap(), "/");
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

#[test]
fn test_into_parts_with_full_uri() {
    use http::uri::{Uri, Parts};
    let uri: Uri = "http://example.com/foo?bar=baz".parse().unwrap();
    let parts = uri.into_parts();
    
    assert_eq!(parts.path_and_query.unwrap(), "/foo?bar=baz");
    assert_eq!(parts.scheme.as_ref().map(|s| s.as_str()), Some("http"));
    assert_eq!(parts.authority.as_ref().map(|a| a.as_str()), Some("example.com"));
}

