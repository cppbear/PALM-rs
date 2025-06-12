// Answer 0

#[test]
fn test_invalid_uri_char_error() {
    let error_kind = InvalidUri(ErrorKind::InvalidUriChar);
    assert_eq!(error_kind.s(), "invalid uri character");
}

#[test]
fn test_invalid_scheme_error() {
    let error_kind = InvalidUri(ErrorKind::InvalidScheme);
    assert_eq!(error_kind.s(), "invalid scheme");
}

#[test]
fn test_invalid_authority_error() {
    let error_kind = InvalidUri(ErrorKind::InvalidAuthority);
    assert_eq!(error_kind.s(), "invalid authority");
}

#[test]
fn test_invalid_port_error() {
    let error_kind = InvalidUri(ErrorKind::InvalidPort);
    assert_eq!(error_kind.s(), "invalid port");
}

#[test]
fn test_invalid_format_error() {
    let error_kind = InvalidUri(ErrorKind::InvalidFormat);
    assert_eq!(error_kind.s(), "invalid format");
}

#[test]
fn test_scheme_missing_error() {
    let error_kind = InvalidUri(ErrorKind::SchemeMissing);
    assert_eq!(error_kind.s(), "scheme missing");
}

#[test]
fn test_authority_missing_error() {
    let error_kind = InvalidUri(ErrorKind::AuthorityMissing);
    assert_eq!(error_kind.s(), "authority missing");
}

#[test]
fn test_path_and_query_missing_error() {
    let error_kind = InvalidUri(ErrorKind::PathAndQueryMissing);
    assert_eq!(error_kind.s(), "path missing");
}

#[test]
fn test_too_long_error() {
    let error_kind = InvalidUri(ErrorKind::TooLong);
    assert_eq!(error_kind.s(), "uri too long");
}

#[test]
fn test_empty_error() {
    let error_kind = InvalidUri(ErrorKind::Empty);
    assert_eq!(error_kind.s(), "empty string");
}

#[test]
fn test_scheme_too_long_error() {
    let error_kind = InvalidUri(ErrorKind::SchemeTooLong);
    assert_eq!(error_kind.s(), "scheme too long");
}

