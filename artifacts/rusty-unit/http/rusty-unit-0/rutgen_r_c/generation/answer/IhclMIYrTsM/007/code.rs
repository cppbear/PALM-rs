// Answer 0

#[test]
fn test_invalid_format_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let invalid_format_error = InvalidUri::new(ErrorKind::InvalidFormat);
    assert_eq!(invalid_format_error.s(), "invalid format");
}

#[test]
fn test_invalid_uri_char_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let invalid_uri_char_error = InvalidUri::new(ErrorKind::InvalidUriChar);
    assert_eq!(invalid_uri_char_error.s(), "invalid uri character");
}

#[test]
fn test_invalid_scheme_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let invalid_scheme_error = InvalidUri::new(ErrorKind::InvalidScheme);
    assert_eq!(invalid_scheme_error.s(), "invalid scheme");
}

#[test]
fn test_invalid_authority_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let invalid_authority_error = InvalidUri::new(ErrorKind::InvalidAuthority);
    assert_eq!(invalid_authority_error.s(), "invalid authority");
}

#[test]
fn test_invalid_port_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let invalid_port_error = InvalidUri::new(ErrorKind::InvalidPort);
    assert_eq!(invalid_port_error.s(), "invalid port");
}

#[test]
fn test_scheme_missing_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let scheme_missing_error = InvalidUri::new(ErrorKind::SchemeMissing);
    assert_eq!(scheme_missing_error.s(), "scheme missing");
}

#[test]
fn test_authority_missing_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let authority_missing_error = InvalidUri::new(ErrorKind::AuthorityMissing);
    assert_eq!(authority_missing_error.s(), "authority missing");
}

#[test]
fn test_path_and_query_missing_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let path_and_query_missing_error = InvalidUri::new(ErrorKind::PathAndQueryMissing);
    assert_eq!(path_and_query_missing_error.s(), "path missing");
}

#[test]
fn test_too_long_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let too_long_error = InvalidUri::new(ErrorKind::TooLong);
    assert_eq!(too_long_error.s(), "uri too long");
}

#[test]
fn test_empty_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let empty_error = InvalidUri::new(ErrorKind::Empty);
    assert_eq!(empty_error.s(), "empty string");
}

#[test]
fn test_scheme_too_long_error() {
    struct TestError(ErrorKind);

    impl InvalidUri {
        fn new(error: ErrorKind) -> Self {
            InvalidUri(TestError(error))
        }
    }

    let scheme_too_long_error = InvalidUri::new(ErrorKind::SchemeTooLong);
    assert_eq!(scheme_too_long_error.s(), "scheme too long");
}

