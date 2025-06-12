// Answer 0

#[derive(Debug)]
enum ErrorKind {
    InvalidUriChar,
    InvalidScheme,
    InvalidAuthority,
    InvalidPort,
    InvalidFormat,
    SchemeMissing,
    AuthorityMissing,
    PathAndQueryMissing,
    TooLong,
    Empty,
    SchemeTooLong,
}

struct UriError(ErrorKind);

impl UriError {
    fn s(&self) -> &str {
        match self.0 {
            ErrorKind::InvalidUriChar => "invalid uri character",
            ErrorKind::InvalidScheme => "invalid scheme",
            ErrorKind::InvalidAuthority => "invalid authority",
            ErrorKind::InvalidPort => "invalid port",
            ErrorKind::InvalidFormat => "invalid format",
            ErrorKind::SchemeMissing => "scheme missing",
            ErrorKind::AuthorityMissing => "authority missing",
            ErrorKind::PathAndQueryMissing => "path missing",
            ErrorKind::TooLong => "uri too long",
            ErrorKind::Empty => "empty string",
            ErrorKind::SchemeTooLong => "scheme too long",
        }
    }
}

#[test]
fn test_invalid_format() {
    let error = UriError(ErrorKind::InvalidFormat);
    assert_eq!(error.s(), "invalid format");
}

#[test]
fn test_invalid_uri_character() {
    let error = UriError(ErrorKind::InvalidUriChar);
    assert_eq!(error.s(), "invalid uri character");
}

#[test]
fn test_invalid_scheme() {
    let error = UriError(ErrorKind::InvalidScheme);
    assert_eq!(error.s(), "invalid scheme");
}

#[test]
fn test_invalid_authority() {
    let error = UriError(ErrorKind::InvalidAuthority);
    assert_eq!(error.s(), "invalid authority");
}

#[test]
fn test_invalid_port() {
    let error = UriError(ErrorKind::InvalidPort);
    assert_eq!(error.s(), "invalid port");
}

#[test]
fn test_scheme_missing() {
    let error = UriError(ErrorKind::SchemeMissing);
    assert_eq!(error.s(), "scheme missing");
}

#[test]
fn test_authority_missing() {
    let error = UriError(ErrorKind::AuthorityMissing);
    assert_eq!(error.s(), "authority missing");
}

#[test]
fn test_path_and_query_missing() {
    let error = UriError(ErrorKind::PathAndQueryMissing);
    assert_eq!(error.s(), "path missing");
}

#[test]
fn test_too_long() {
    let error = UriError(ErrorKind::TooLong);
    assert_eq!(error.s(), "uri too long");
}

#[test]
fn test_empty() {
    let error = UriError(ErrorKind::Empty);
    assert_eq!(error.s(), "empty string");
}

#[test]
fn test_scheme_too_long() {
    let error = UriError(ErrorKind::SchemeTooLong);
    assert_eq!(error.s(), "scheme too long");
}

