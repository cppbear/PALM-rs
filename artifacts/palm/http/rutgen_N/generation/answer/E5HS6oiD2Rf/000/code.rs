// Answer 0

#[test]
fn test_from_parts_relative_uri() {
    use http::uri::*;

    let mut parts = Parts::default();
    parts.path_and_query = Some("/foo".parse().unwrap());

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.path(), "/foo");
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
}

#[test]
fn test_from_parts_absolute_uri() {
    use http::uri::*;

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
#[should_panic]
fn test_from_parts_missing_authority() {
    use http::uri::*;

    let mut parts = Parts::default();
    parts.scheme = Some("http".parse().unwrap());
    parts.path_and_query = Some("/foo".parse().unwrap());

    let _uri = Uri::from_parts(parts).unwrap();
}

#[test]
#[should_panic]
fn test_from_parts_missing_path_and_query() {
    use http::uri::*;

    let mut parts = Parts::default();
    parts.scheme = Some("http".parse().unwrap());
    parts.authority = Some("foo.com".parse().unwrap());

    let _uri = Uri::from_parts(parts).unwrap();
}

#[test]
#[should_panic]
fn test_from_parts_scheme_missing_with_authority_and_path() {
    use http::uri::*;

    let mut parts = Parts::default();
    parts.authority = Some("foo.com".parse().unwrap());
    parts.path_and_query = Some("/foo".parse().unwrap());

    let _uri = Uri::from_parts(parts).unwrap();
}

