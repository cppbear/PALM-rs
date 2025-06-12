// Answer 0

#[test]
fn test_invalid_uri_scheme_missing() {
    let error_kind = ErrorKind::SchemeMissing;
    let invalid_uri = InvalidUri(error_kind);
    invalid_uri.s();
}

#[test]
fn test_invalid_uri_scheme_too_long() {
    let error_kind = ErrorKind::SchemeTooLong;
    let invalid_uri = InvalidUri(error_kind);
    invalid_uri.s();
}

#[test]
fn test_invalid_uri_empty() {
    let error_kind = ErrorKind::Empty;
    let invalid_uri = InvalidUri(error_kind);
    invalid_uri.s();
}

