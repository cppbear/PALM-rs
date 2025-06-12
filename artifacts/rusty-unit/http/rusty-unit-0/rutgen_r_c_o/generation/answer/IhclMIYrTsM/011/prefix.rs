// Answer 0

#[test]
fn test_invalid_uri_char() {
    let error_invalid_uri_char = InvalidUri(ErrorKind::InvalidUriChar);
    let _ = error_invalid_uri_char.s();
}

#[test]
fn test_invalid_scheme() {
    let error_invalid_scheme = InvalidUri(ErrorKind::InvalidScheme);
    let _ = error_invalid_scheme.s();
}

#[test]
fn test_invalid_authority() {
    let error_invalid_authority = InvalidUri(ErrorKind::InvalidAuthority);
    let _ = error_invalid_authority.s();
}

#[test]
fn test_invalid_port() {
    let error_invalid_port = InvalidUri(ErrorKind::InvalidPort);
    let _ = error_invalid_port.s();
}

#[test]
fn test_invalid_format() {
    let error_invalid_format = InvalidUri(ErrorKind::InvalidFormat);
    let _ = error_invalid_format.s();
}

#[test]
fn test_scheme_missing() {
    let error_scheme_missing = InvalidUri(ErrorKind::SchemeMissing);
    let _ = error_scheme_missing.s();
}

#[test]
fn test_authority_missing() {
    let error_authority_missing = InvalidUri(ErrorKind::AuthorityMissing);
    let _ = error_authority_missing.s();
}

#[test]
fn test_path_and_query_missing() {
    let error_path_and_query_missing = InvalidUri(ErrorKind::PathAndQueryMissing);
    let _ = error_path_and_query_missing.s();
}

#[test]
fn test_too_long() {
    let error_too_long = InvalidUri(ErrorKind::TooLong);
    let _ = error_too_long.s();
}

#[test]
fn test_empty() {
    let error_empty = InvalidUri(ErrorKind::Empty);
    let _ = error_empty.s();
}

#[test]
fn test_scheme_too_long() {
    let error_scheme_too_long = InvalidUri(ErrorKind::SchemeTooLong);
    let _ = error_scheme_too_long.s();
}

