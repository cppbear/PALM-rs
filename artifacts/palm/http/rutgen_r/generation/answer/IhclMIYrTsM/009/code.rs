// Answer 0

struct ErrorKindWrapper(ErrorKind);

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

impl ErrorKindWrapper {
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
fn test_invalid_authority() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::InvalidAuthority);
    assert_eq!(error_wrapper.s(), "invalid authority");
}

#[test]
fn test_invalid_uri_char() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::InvalidUriChar);
    assert_eq!(error_wrapper.s(), "invalid uri character");
}

#[test]
fn test_invalid_scheme() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::InvalidScheme);
    assert_eq!(error_wrapper.s(), "invalid scheme");
}

#[test]
fn test_invalid_port() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::InvalidPort);
    assert_eq!(error_wrapper.s(), "invalid port");
}

#[test]
fn test_invalid_format() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::InvalidFormat);
    assert_eq!(error_wrapper.s(), "invalid format");
}

#[test]
fn test_scheme_missing() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::SchemeMissing);
    assert_eq!(error_wrapper.s(), "scheme missing");
}

#[test]
fn test_authority_missing() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::AuthorityMissing);
    assert_eq!(error_wrapper.s(), "authority missing");
}

#[test]
fn test_path_and_query_missing() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::PathAndQueryMissing);
    assert_eq!(error_wrapper.s(), "path missing");
}

#[test]
fn test_too_long() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::TooLong);
    assert_eq!(error_wrapper.s(), "uri too long");
}

#[test]
fn test_empty_string() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::Empty);
    assert_eq!(error_wrapper.s(), "empty string");
}

#[test]
fn test_scheme_too_long() {
    let error_wrapper = ErrorKindWrapper(ErrorKind::SchemeTooLong);
    assert_eq!(error_wrapper.s(), "scheme too long");
}

