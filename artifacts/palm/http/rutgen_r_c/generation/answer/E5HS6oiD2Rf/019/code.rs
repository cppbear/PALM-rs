// Answer 0

#[test]
fn test_from_parts_with_authority_only() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::from_static("foo.com"));

    let uri = Uri::from_parts(parts).unwrap();

    assert!(uri.scheme().is_none());
    assert_eq!(uri.authority().unwrap().as_str(), "foo.com");
    assert!(uri.path_and_query().is_none());
}

#[test]
fn test_from_parts_with_scheme_and_authority() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard("http".into()) });
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.scheme().unwrap().as_str(), "http");
    assert_eq!(uri.authority().unwrap().as_str(), "foo.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
fn test_from_parts_with_empty_parts() {
    let parts = Parts::default();
    let result = Uri::from_parts(parts);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, ErrorKind::SchemeMissing);
}

#[test]
fn test_from_parts_with_only_scheme() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard("http".into()) });
    
    let result = Uri::from_parts(parts);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, ErrorKind::AuthorityMissing);
}

#[test]
fn test_from_parts_with_only_path_and_query() {
    let mut parts = Parts::default();
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let result = Uri::from_parts(parts);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, ErrorKind::SchemeMissing);
}

