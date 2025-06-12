// Answer 0

#[test]
fn test_invalid_uri_scheme_too_long() {
    struct TestInvalidUri {
        kind: ErrorKind,
    }

    impl InvalidUri {
        fn new(kind: ErrorKind) -> Self {
            TestInvalidUri { kind }
        }
    }

    let error = InvalidUri::new(ErrorKind::SchemeTooLong);
    assert_eq!(error.s(), "scheme too long");
}

