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
fn test_from_parts_empty_uri() {
    use http::uri::*;

    let parts = Parts::default(); // All fields will be None

    let uri = Uri::from_parts(parts).unwrap();

    assert_eq!(uri.scheme().inner, Scheme2::None);
    assert!(uri.authority().is_empty());
    assert!(uri.path_and_query.is_empty());
}

