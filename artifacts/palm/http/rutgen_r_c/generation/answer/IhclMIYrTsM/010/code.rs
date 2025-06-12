// Answer 0

#[test]
fn test_invalid_scheme() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        InvalidScheme,
    }
    
    let error = InvalidUri(ErrorKind::InvalidScheme);
    assert_eq!(error.s(), "invalid scheme");
}

#[test]
fn test_invalid_uri_char() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        InvalidUriChar,
    }
    
    let error = InvalidUri(ErrorKind::InvalidUriChar);
    assert_eq!(error.s(), "invalid uri character");
}

#[test]
fn test_invalid_authority() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        InvalidAuthority,
    }
    
    let error = InvalidUri(ErrorKind::InvalidAuthority);
    assert_eq!(error.s(), "invalid authority");
}

#[test]
fn test_invalid_port() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        InvalidPort,
    }
    
    let error = InvalidUri(ErrorKind::InvalidPort);
    assert_eq!(error.s(), "invalid port");
}

#[test]
fn test_invalid_format() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        InvalidFormat,
    }
    
    let error = InvalidUri(ErrorKind::InvalidFormat);
    assert_eq!(error.s(), "invalid format");
}

#[test]
fn test_scheme_missing() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        SchemeMissing,
    }
    
    let error = InvalidUri(ErrorKind::SchemeMissing);
    assert_eq!(error.s(), "scheme missing");
}

#[test]
fn test_authority_missing() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        AuthorityMissing,
    }
    
    let error = InvalidUri(ErrorKind::AuthorityMissing);
    assert_eq!(error.s(), "authority missing");
}

#[test]
fn test_path_and_query_missing() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        PathAndQueryMissing,
    }
    
    let error = InvalidUri(ErrorKind::PathAndQueryMissing);
    assert_eq!(error.s(), "path missing");
}

#[test]
fn test_too_long() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        TooLong,
    }
    
    let error = InvalidUri(ErrorKind::TooLong);
    assert_eq!(error.s(), "uri too long");
}

#[test]
fn test_empty() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        Empty,
    }
    
    let error = InvalidUri(ErrorKind::Empty);
    assert_eq!(error.s(), "empty string");
}

#[test]
fn test_scheme_too_long() {
    struct DummyStatusCode;
    struct DummyMethod;
    struct DummyUri;

    impl Error for DummyStatusCode {}
    impl Error for DummyMethod {}
    impl Error for DummyUri {}

    struct InvalidUri(ErrorKind);
    
    #[derive(Debug, Eq, PartialEq)]
    enum ErrorKind {
        SchemeTooLong,
    }
    
    let error = InvalidUri(ErrorKind::SchemeTooLong);
    assert_eq!(error.s(), "scheme too long");
}

