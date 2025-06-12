// Answer 0

#[test]
fn test_into_parts_with_valid_uri() {
    use crate::uri::*;
    let uri: Uri = "/foo".parse().unwrap();
    
    let parts = uri.into_parts();
    
    assert_eq!(parts.path_and_query.unwrap().data, ByteStr::from_static(b"/foo"));
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

#[test]
fn test_into_parts_with_empty_path() {
    use crate::uri::*;
    let uri: Uri = "".parse().unwrap();
    
    let parts = uri.into_parts();
    
    assert!(parts.path_and_query.is_none());
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

#[test]
fn test_into_parts_with_static_uri() {
    use crate::uri::*;
    let uri: Uri = "http://example.com".parse().unwrap();
    
    let parts = uri.into_parts();
    
    assert_eq!(parts.scheme.unwrap().inner, Scheme2::from_str("http").unwrap());
    assert_eq!(parts.authority.unwrap().data, ByteStr::from_static(b"example.com"));
    assert_eq!(parts.path_and_query.unwrap().data, ByteStr::from_static(b""));
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_into_parts_with_invalid_static_uri() {
    use crate::uri::*;
    let uri: Uri = "invalid_uri".parse().unwrap();
    
    let _parts = uri.into_parts(); // This should panic
}

