// Answer 0

#[test]
fn test_from_parts_absolute_uri() {
    use http::uri::{Authority, PathAndQuery, Scheme, Uri, Parts};

    let mut parts = Parts::default();
    parts.scheme = Some("http".parse().unwrap());
    parts.authority = Some("foo.com".parse().unwrap());
    parts.path_and_query = Some("/foo".parse().unwrap());

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.scheme().unwrap().as_str(), "http");
    assert_eq!(uri.authority().unwrap(), "foo.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
fn test_from_parts_relative_uri() {
    use http::uri::{PathAndQuery, Uri, Parts};

    let mut parts = Parts::default();
    parts.path_and_query = Some("/foo".parse().unwrap());

    // This should panic, as scheme and authority cannot be none together.
    assert!(Uri::from_parts(parts).is_err());
}

#[test]
fn test_from_parts_scheme_missing() {
    use http::uri::{Authority, PathAndQuery, Uri, Parts};

    let mut parts = Parts::default();
    parts.authority = Some("foo.com".parse().unwrap());
    parts.path_and_query = Some("/foo".parse().unwrap());

    // This should return an error because scheme is missing
    assert!(Uri::from_parts(parts).is_err());
}

#[test]
fn test_from_parts_authority_missing() {
    use http::uri::{Scheme, PathAndQuery, Uri, Parts};

    let mut parts = Parts::default();
    parts.scheme = Some("http".parse().unwrap());
    parts.path_and_query = Some("/foo".parse().unwrap());

    // This should return an error because authority is missing
    assert!(Uri::from_parts(parts).is_err());
}

