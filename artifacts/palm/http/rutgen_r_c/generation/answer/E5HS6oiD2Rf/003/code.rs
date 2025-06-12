// Answer 0

#[test]
fn test_from_parts_absolute_uri() {
    use http::uri::*;
    
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Http) });
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));
    
    let uri = Uri::from_parts(parts).unwrap();
    
    assert_eq!(uri.scheme().unwrap().as_str(), "http");
    assert_eq!(uri.authority().unwrap().as_str(), "foo.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
fn test_from_parts_relative_uri() {
    use http::uri::*;
    
    let mut parts = Parts::default();
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));
    
    let uri = Uri::from_parts(parts).unwrap();
    
    assert_eq!(uri.path(), "/foo");
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
}

#[test]
#[should_panic]
fn test_from_parts_scheme_missing() {
    use http::uri::*;
    
    let mut parts = Parts::default();
    parts.scheme = None;
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));
    
    let _uri = Uri::from_parts(parts).unwrap();
}

#[test]
#[should_panic]
fn test_from_parts_authority_missing() {
    use http::uri::*;
    
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Http) });
    parts.authority = None;
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));
    
    let _uri = Uri::from_parts(parts).unwrap();
}

#[test]
#[should_panic]
fn test_from_parts_path_and_query_missing() {
    use http::uri::*;
    
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Http) });
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = None;
    
    let _uri = Uri::from_parts(parts).unwrap();
}

