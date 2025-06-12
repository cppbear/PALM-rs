// Answer 0

#[test]
fn test_invalid_scheme() {
    let invalid_uri_error = InvalidUri(ErrorKind::InvalidScheme);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_empty_string() {
    let invalid_uri_error = InvalidUri(ErrorKind::Empty);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_invalid_uri_character() {
    let invalid_uri_error = InvalidUri(ErrorKind::InvalidUriChar);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_invalid_authority() {
    let invalid_uri_error = InvalidUri(ErrorKind::InvalidAuthority);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_invalid_port() {
    let invalid_uri_error = InvalidUri(ErrorKind::InvalidPort);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_invalid_format() {
    let invalid_uri_error = InvalidUri(ErrorKind::InvalidFormat);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_authority_missing() {
    let invalid_uri_error = InvalidUri(ErrorKind::AuthorityMissing);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_scheme_missing() {
    let invalid_uri_error = InvalidUri(ErrorKind::SchemeMissing);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_path_and_query_missing() {
    let invalid_uri_error = InvalidUri(ErrorKind::PathAndQueryMissing);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_too_long() {
    let invalid_uri_error = InvalidUri(ErrorKind::TooLong);
    let _ = invalid_uri_error.s();
}

#[test]
fn test_scheme_too_long() {
    let invalid_uri_error = InvalidUri(ErrorKind::SchemeTooLong);
    let _ = invalid_uri_error.s();
}

