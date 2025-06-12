// Answer 0

#[test]
fn test_from_parts_absolute_uri() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme::from_static("http"));
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.scheme_str(), Some("http"));
    assert_eq!(uri.authority().unwrap().as_str(), "foo.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
fn test_from_parts_relative_uri() {
    let mut parts = Parts::default();
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.path(), "/foo");
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
}

#[test]
fn test_from_parts_missing_authority() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme::from_static("http"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let result = Uri::from_parts(parts);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().0, ErrorKind::AuthorityMissing);
}

#[test]
fn test_from_parts_missing_path_and_query() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme::from_static("http"));
    parts.authority = Some(Authority::from_static("foo.com"));

    let result = Uri::from_parts(parts);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().0, ErrorKind::PathAndQueryMissing);
}

#[test]
fn test_from_parts_missing_scheme() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let result = Uri::from_parts(parts);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().0, ErrorKind::SchemeMissing);
}

