// Answer 0

#[test]
fn test_is_with_invalid_uri_char() {
    struct InvalidUriChar;
    impl error::Error for InvalidUriChar {}

    let error = Error {
        inner: ErrorKind::InvalidUriChar,
    };

    assert!(error.is::<InvalidUriChar>());
}

#[test]
fn test_is_with_invalid_scheme() {
    struct InvalidScheme;
    impl error::Error for InvalidScheme {}

    let error = Error {
        inner: ErrorKind::InvalidScheme,
    };

    assert!(error.is::<InvalidScheme>());
}

#[test]
fn test_is_with_invalid_authority() {
    struct InvalidAuthority;
    impl error::Error for InvalidAuthority {}

    let error = Error {
        inner: ErrorKind::InvalidAuthority,
    };

    assert!(error.is::<InvalidAuthority>());
}

#[test]
fn test_is_with_invalid_port() {
    struct InvalidPort;
    impl error::Error for InvalidPort {}

    let error = Error {
        inner: ErrorKind::InvalidPort,
    };

    assert!(error.is::<InvalidPort>());
}

#[test]
fn test_is_with_invalid_format() {
    struct InvalidFormat;
    impl error::Error for InvalidFormat {}

    let error = Error {
        inner: ErrorKind::InvalidFormat,
    };

    assert!(error.is::<InvalidFormat>());
}

#[test]
fn test_is_with_scheme_missing() {
    struct SchemeMissing;
    impl error::Error for SchemeMissing {}

    let error = Error {
        inner: ErrorKind::SchemeMissing,
    };

    assert!(error.is::<SchemeMissing>());
}

#[test]
fn test_is_with_authority_missing() {
    struct AuthorityMissing;
    impl error::Error for AuthorityMissing {}

    let error = Error {
        inner: ErrorKind::AuthorityMissing,
    };

    assert!(error.is::<AuthorityMissing>());
}

#[test]
fn test_is_with_path_and_query_missing() {
    struct PathAndQueryMissing;
    impl error::Error for PathAndQueryMissing {}

    let error = Error {
        inner: ErrorKind::PathAndQueryMissing,
    };

    assert!(error.is::<PathAndQueryMissing>());
}

#[test]
fn test_is_with_too_long() {
    struct TooLong;
    impl error::Error for TooLong {}

    let error = Error {
        inner: ErrorKind::TooLong,
    };

    assert!(error.is::<TooLong>());
}

#[test]
fn test_is_with_empty() {
    struct Empty;
    impl error::Error for Empty {}

    let error = Error {
        inner: ErrorKind::Empty,
    };

    assert!(error.is::<Empty>());
}

#[test]
fn test_is_with_scheme_too_long() {
    struct SchemeTooLong;
    impl error::Error for SchemeTooLong {}

    let error = Error {
        inner: ErrorKind::SchemeTooLong,
    };

    assert!(error.is::<SchemeTooLong>());
}

