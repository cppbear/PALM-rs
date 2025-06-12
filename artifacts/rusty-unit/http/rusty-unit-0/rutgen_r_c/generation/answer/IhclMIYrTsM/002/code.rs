// Answer 0

#[test]
fn test_invalid_uri_empty_string() {
    use std::error::Error;

    struct InvalidUri(ErrorKind);

    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        Empty,
    }

    impl InvalidUri {
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

    let error_instance = InvalidUri(ErrorKind::Empty);
    assert_eq!(error_instance.s(), "empty string");
}

#[test]
fn test_invalid_uri_too_long() {
    struct InvalidUri(ErrorKind);

    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        TooLong,
    }

    impl InvalidUri {
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

    let error_instance = InvalidUri(ErrorKind::TooLong);
    assert_eq!(error_instance.s(), "uri too long");
}

