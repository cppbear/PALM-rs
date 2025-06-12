// Answer 0

#[test]
fn test_invalid_uri_char() {
    let error = InvalidUri(ErrorKind::InvalidUriChar);
    assert_eq!(error.s(), "invalid uri character");
}

#[test]
fn test_invalid_scheme() {
    let error = InvalidUri(ErrorKind::InvalidScheme);
    assert_eq!(error.s(), "invalid scheme");
}

#[test]
fn test_invalid_authority() {
    let error = InvalidUri(ErrorKind::InvalidAuthority);
    assert_eq!(error.s(), "invalid authority");
}

#[test]
fn test_invalid_port() {
    let error = InvalidUri(ErrorKind::InvalidPort);
    assert_eq!(error.s(), "invalid port");
}

#[test]
fn test_invalid_format() {
    let error = InvalidUri(ErrorKind::InvalidFormat);
    assert_eq!(error.s(), "invalid format");
}

#[test]
fn test_scheme_missing() {
    let error = InvalidUri(ErrorKind::SchemeMissing);
    assert_eq!(error.s(), "scheme missing");
}

#[test]
fn test_authority_missing() {
    let error = InvalidUri(ErrorKind::AuthorityMissing);
    assert_eq!(error.s(), "authority missing");
}

#[test]
fn test_path_and_query_missing() {
    let error = InvalidUri(ErrorKind::PathAndQueryMissing);
    assert_eq!(error.s(), "path missing");
}

#[test]
fn test_too_long() {
    let error = InvalidUri(ErrorKind::TooLong);
    assert_eq!(error.s(), "uri too long");
}

#[test]
fn test_empty() {
    let error = InvalidUri(ErrorKind::Empty);
    assert_eq!(error.s(), "empty string");
}

#[test]
fn test_scheme_too_long() {
    let error = InvalidUri(ErrorKind::SchemeTooLong);
    assert_eq!(error.s(), "scheme too long");
}

