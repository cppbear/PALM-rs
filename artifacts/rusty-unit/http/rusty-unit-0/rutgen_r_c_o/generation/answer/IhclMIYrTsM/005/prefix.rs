// Answer 0

#[test]
fn test_invalid_uri_authority_missing() {
    let error_kind = ErrorKind::AuthorityMissing;
    let invalid_uri = InvalidUri(error_kind);
    invalid_uri.s();
}

