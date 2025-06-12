// Answer 0

struct ErrorKind {
    kind: ErrorVariant,
}

enum ErrorVariant {
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

impl ErrorKind {
    fn s(&self) -> &str {
        match self.kind {
            ErrorVariant::InvalidUriChar => "invalid uri character",
            ErrorVariant::InvalidScheme => "invalid scheme",
            ErrorVariant::InvalidAuthority => "invalid authority",
            ErrorVariant::InvalidPort => "invalid port",
            ErrorVariant::InvalidFormat => "invalid format",
            ErrorVariant::SchemeMissing => "scheme missing",
            ErrorVariant::AuthorityMissing => "authority missing",
            ErrorVariant::PathAndQueryMissing => "path missing",
            ErrorVariant::TooLong => "uri too long",
            ErrorVariant::Empty => "empty string",
            ErrorVariant::SchemeTooLong => "scheme too long",
        }
    }
}

#[test]
fn test_invalid_uri_character() {
    let error = ErrorKind { kind: ErrorVariant::InvalidUriChar };
    assert_eq!(error.s(), "invalid uri character");
}

#[test]
fn test_invalid_scheme() {
    let error = ErrorKind { kind: ErrorVariant::InvalidScheme };
    assert_eq!(error.s(), "invalid scheme");
}

#[test]
fn test_invalid_authority() {
    let error = ErrorKind { kind: ErrorVariant::InvalidAuthority };
    assert_eq!(error.s(), "invalid authority");
}

#[test]
fn test_invalid_port() {
    let error = ErrorKind { kind: ErrorVariant::InvalidPort };
    assert_eq!(error.s(), "invalid port");
}

#[test]
fn test_invalid_format() {
    let error = ErrorKind { kind: ErrorVariant::InvalidFormat };
    assert_eq!(error.s(), "invalid format");
}

#[test]
fn test_scheme_missing() {
    let error = ErrorKind { kind: ErrorVariant::SchemeMissing };
    assert_eq!(error.s(), "scheme missing");
}

#[test]
fn test_authority_missing() {
    let error = ErrorKind { kind: ErrorVariant::AuthorityMissing };
    assert_eq!(error.s(), "authority missing");
}

#[test]
fn test_path_and_query_missing() {
    let error = ErrorKind { kind: ErrorVariant::PathAndQueryMissing };
    assert_eq!(error.s(), "path missing");
}

#[test]
fn test_too_long() {
    let error = ErrorKind { kind: ErrorVariant::TooLong };
    assert_eq!(error.s(), "uri too long");
}

#[test]
fn test_empty() {
    let error = ErrorKind { kind: ErrorVariant::Empty };
    assert_eq!(error.s(), "empty string");
}

#[test]
fn test_scheme_too_long() {
    let error = ErrorKind { kind: ErrorVariant::SchemeTooLong };
    assert_eq!(error.s(), "scheme too long");
}

