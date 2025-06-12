// Answer 0

#[test]
fn test_from_parts_with_scheme_and_authority_missing_path() {
    use http::uri::*;

    let mut parts = Parts::default();
    parts.scheme = Some("http".parse().unwrap());
    parts.authority = Some("foo.com".parse().unwrap());
    parts.path_and_query = None;

    let result = Uri::from_parts(parts);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), &ErrorKind::PathAndQueryMissing);
}

