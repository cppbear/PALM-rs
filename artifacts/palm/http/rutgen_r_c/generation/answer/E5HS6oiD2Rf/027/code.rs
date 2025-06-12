// Answer 0

#[test]
fn test_uri_from_parts_empty() {
    let parts = Parts::default();
    let uri = Uri::from_parts(parts).unwrap();
    
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
    assert_eq!(uri.path(), "");
}

#[test]
fn test_uri_from_parts_with_scheme_none_authority_none_path_and_query_empty() {
    let mut parts = Parts::default();
    parts.path_and_query = Some(PathAndQuery::empty());
    
    let uri = Uri::from_parts(parts).unwrap();
    
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
    assert_eq!(uri.path_and_query().unwrap().as_str(), "");
}

#[test]
fn test_uri_from_parts_with_no_scheme_and_authority() {
    let mut parts = Parts::default();
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));
    
    let uri = Uri::from_parts(parts).unwrap();
    
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
    assert_eq!(uri.path(), "/foo");
}

