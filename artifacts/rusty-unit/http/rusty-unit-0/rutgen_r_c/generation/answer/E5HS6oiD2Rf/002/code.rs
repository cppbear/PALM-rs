// Answer 0

#[test]
fn test_from_parts_scheme_missing() {
    // Create a `Parts` instance with a scheme and authority but no path_and_query.
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme {
        inner: Scheme2::Standard(Protocol::Http),
    });
    parts.authority = Some(Authority::from_static("example.com"));

    // Assert that an Err is returned due to the missing path_and_query.
    let result = Uri::from_parts(parts);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.0, ErrorKind::PathAndQueryMissing);
    }
}

