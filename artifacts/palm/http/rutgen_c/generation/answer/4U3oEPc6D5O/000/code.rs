// Answer 0

#[test]
fn test_map_success() {
    #[derive(Debug)]
    struct MockParts {
        _priv: (),
    }

    #[derive(Debug)]
    struct MockError;

    impl From<MockError> for crate::Error {
        fn from(_: MockError) -> Self {
            crate::Error { inner: ErrorKind::SomeErrorKind }
        }
    }

    let builder = Builder { parts: Ok(MockParts { _priv: () }) };
    let result = builder.map(|parts| {
        Ok(parts)
    });

    assert!(result.parts.is_ok());
}

#[test]
fn test_map_failure() {
    #[derive(Debug)]
    struct MockParts {
        _priv: (),
    }

    #[derive(Debug)]
    struct MockError;

    impl From<MockError> for crate::Error {
        fn from(_: MockError) -> Self {
            crate::Error { inner: ErrorKind::SomeErrorKind }
        }
    }

    let builder = Builder { parts: Ok(MockParts { _priv: () }) };
    let result = builder.map(|_| {
        Err(MockError.into())
    });

    assert!(result.parts.is_err());
}

