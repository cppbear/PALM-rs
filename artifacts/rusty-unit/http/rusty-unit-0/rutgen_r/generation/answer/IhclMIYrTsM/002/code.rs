// Answer 0

fn test_s_empty_string() {
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

    let error = UriError(ErrorKind::Empty);
    assert_eq!(error.s(), "empty string");
}

