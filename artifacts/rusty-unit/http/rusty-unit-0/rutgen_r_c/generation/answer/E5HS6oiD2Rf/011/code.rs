// Answer 0

#[test]
fn test_from_parts_scheme_missing() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let result = Uri::from_parts(parts);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, ErrorKind::SchemeMissing);
}

