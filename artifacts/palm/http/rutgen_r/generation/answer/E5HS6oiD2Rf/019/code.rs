// Answer 0

#[test]
fn test_from_parts_relative_uri() {
    use http::uri::{Parts, Uri, Scheme, Authority, PathAndQuery};

    let mut parts = Parts::default();
    parts.path_and_query = Some(PathAndQuery::from_str("/foo").unwrap());

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.path(), "/foo");
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
}

#[test]
fn test_from_parts_absolute_uri() {
    use http::uri::{Parts, Uri, Scheme, Authority, PathAndQuery};

    let mut parts = Parts::default();
    parts.scheme = Some(Scheme::from_str("http").unwrap());
    parts.authority = Some(Authority::from_str("foo.com").unwrap());
    parts.path_and_query = Some(PathAndQuery::from_str("/foo").unwrap());

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.scheme().unwrap().as_str(), "http");
    assert_eq!(uri.authority().unwrap(), "foo.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
#[should_panic]
fn test_from_parts_missing_scheme_with_authority() {
    use http::uri::{Parts, Uri, Authority};

    let mut parts = Parts::default();
    parts.authority = Some(Authority::from_str("foo.com").unwrap());

    let uri = Uri::from_parts(parts).unwrap();
}

#[test]
#[should_panic]
fn test_from_parts_missing_authority_with_scheme() {
    use http::uri::{Parts, Uri, Scheme};

    let mut parts = Parts::default();
    parts.scheme = Some(Scheme::from_str("http").unwrap());
    
    let uri = Uri::from_parts(parts).unwrap();
}

#[test]
#[should_panic]
fn test_from_parts_missing_path_query_with_scheme_and_authority() {
    use http::uri::{Parts, Uri, Scheme, Authority};

    let mut parts = Parts::default();
    parts.scheme = Some(Scheme::from_str("http").unwrap());
    parts.authority = Some(Authority::from_str("foo.com").unwrap());

    let uri = Uri::from_parts(parts).unwrap();
}

