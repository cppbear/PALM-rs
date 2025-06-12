// Answer 0

#[test]
fn test_source_with_error_kind_invalid_url_char() {
    struct TestInvalidUriChar;
    impl error::Error for TestInvalidUriChar {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            None
        }
    }

    let error = Error {
        inner: ErrorKind::InvalidUriChar,
    };

    assert_eq!(error.source(), None);
}

#[test]
fn test_source_with_error_kind_invalid_scheme() {
    struct TestInvalidScheme;
    impl error::Error for TestInvalidScheme {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            Some(&TestInvalidUriChar)
        }
    }

    let error = Error {
        inner: ErrorKind::InvalidScheme,
    };

    assert_eq!(error.source(), Some(&TestInvalidUriChar));
}

#[test]
fn test_source_with_error_kind_invalid_authority() {
    struct TestInvalidAuthority;
    impl error::Error for TestInvalidAuthority {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            None
        }
    }

    let error = Error {
        inner: ErrorKind::InvalidAuthority,
    };

    assert_eq!(error.source(), None);
}

#[test]
fn test_source_with_error_kind_invalid_format() {
    struct TestInvalidFormat;
    impl error::Error for TestInvalidFormat {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            Some(&TestInvalidUriChar)
        }
    }

    let error = Error {
        inner: ErrorKind::InvalidFormat,
    };

    assert_eq!(error.source(), Some(&TestInvalidUriChar));
}

#[test]
fn test_source_with_error_kind_scheme_missing() {
    struct TestSchemeMissing;
    impl error::Error for TestSchemeMissing {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            None
        }
    }

    let error = Error {
        inner: ErrorKind::SchemeMissing,
    };

    assert_eq!(error.source(), None);
}

