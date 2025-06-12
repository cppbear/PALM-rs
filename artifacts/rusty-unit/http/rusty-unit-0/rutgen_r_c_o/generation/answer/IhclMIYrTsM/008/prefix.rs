// Answer 0

#[test]
fn test_invalid_port() {
    let error_kind = ErrorKind::InvalidPort;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_invalid_uri_char() {
    let error_kind = ErrorKind::InvalidUriChar;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_invalid_scheme() {
    let error_kind = ErrorKind::InvalidScheme;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_invalid_authority() {
    let error_kind = ErrorKind::InvalidAuthority;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_invalid_format() {
    let error_kind = ErrorKind::InvalidFormat;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_scheme_missing() {
    let error_kind = ErrorKind::SchemeMissing;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_authority_missing() {
    let error_kind = ErrorKind::AuthorityMissing;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_path_and_query_missing() {
    let error_kind = ErrorKind::PathAndQueryMissing;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_too_long() {
    let error_kind = ErrorKind::TooLong;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_empty() {
    let error_kind = ErrorKind::Empty;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

#[test]
fn test_scheme_too_long() {
    let error_kind = ErrorKind::SchemeTooLong;
    let invalid_uri = InvalidUri(error_kind);
    let result = invalid_uri.s();
}

