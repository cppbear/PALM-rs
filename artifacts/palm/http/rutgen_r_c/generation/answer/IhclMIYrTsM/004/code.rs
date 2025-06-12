// Answer 0

#[test]
fn test_invalid_uri_path_and_query_missing() {
    struct MockInvalidUri(ErrorKind);
    
    let invalid_uri = MockInvalidUri(ErrorKind::PathAndQueryMissing);
    
    assert_eq!(invalid_uri.s(), "path missing");
}

#[test]
fn test_invalid_uri_scheme_missing() {
    struct MockInvalidUri(ErrorKind);
    
    let invalid_uri = MockInvalidUri(ErrorKind::SchemeMissing);
    
    assert_eq!(invalid_uri.s(), "scheme missing");
}

#[test]
fn test_invalid_uri_authority_missing() {
    struct MockInvalidUri(ErrorKind);
    
    let invalid_uri = MockInvalidUri(ErrorKind::AuthorityMissing);
    
    assert_eq!(invalid_uri.s(), "authority missing");
}

