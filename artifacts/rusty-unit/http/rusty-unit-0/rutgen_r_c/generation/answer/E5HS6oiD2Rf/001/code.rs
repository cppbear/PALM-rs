// Answer 0

#[test]
fn test_from_parts_authority_missing() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Http) }); // Ensure scheme is some
    parts.authority = None; // Ensure authority is none
    parts.path_and_query = Some(PathAndQuery::from_static("/foo")); // Optional, can be present

    // This should return an error indicating that authority is missing
    let result = Uri::from_parts(parts);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), ErrorKind::AuthorityMissing.into());
}

