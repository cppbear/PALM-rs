// Answer 0

#[test]
fn test_invalid_authority() {
    let error = InvalidUri(ErrorKind::InvalidAuthority);
    let _ = error.s();
}

#[test]
fn test_invalid_uri_char() {
    let error = InvalidUri(ErrorKind::InvalidUriChar);
    let _ = error.s();
}

#[test]
fn test_invalid_scheme() {
    let error = InvalidUri(ErrorKind::InvalidScheme);
    let _ = error.s();
}

#[test]
fn test_invalid_port() {
    let error = InvalidUri(ErrorKind::InvalidPort);
    let _ = error.s();
}

#[test]
fn test_invalid_format() {
    let error = InvalidUri(ErrorKind::InvalidFormat);
    let _ = error.s();
}

#[test]
fn test_scheme_missing() {
    let error = InvalidUri(ErrorKind::SchemeMissing);
    let _ = error.s();
}

#[test]
fn test_authority_missing() {
    let error = InvalidUri(ErrorKind::AuthorityMissing);
    let _ = error.s();
}

#[test]
fn test_path_and_query_missing() {
    let error = InvalidUri(ErrorKind::PathAndQueryMissing);
    let _ = error.s();
}

#[test]
fn test_too_long() {
    let error = InvalidUri(ErrorKind::TooLong);
    let _ = error.s();
}

#[test]
fn test_empty() {
    let error = InvalidUri(ErrorKind::Empty);
    let _ = error.s();
}

#[test]
fn test_scheme_too_long() {
    let error = InvalidUri(ErrorKind::SchemeTooLong);
    let _ = error.s();
}

