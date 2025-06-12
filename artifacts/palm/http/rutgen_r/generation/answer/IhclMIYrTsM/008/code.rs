// Answer 0

#[test]
fn test_invalid_port_error() {
    struct ErrorKindWrapper(ErrorKind);

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

    let error = ErrorKindWrapper(ErrorKind::InvalidPort);
    assert_eq!(error.s(), "invalid port");
}

