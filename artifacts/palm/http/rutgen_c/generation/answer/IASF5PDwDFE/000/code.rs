// Answer 0

#[test]
fn test_and_then_success() {
    struct TestData;
    impl TryInto<StatusCode> for TestData {
        type Error = crate::Error;

        fn try_into(self) -> Result<StatusCode, Self::Error> {
            Ok(StatusCode::OK)
        }
    }

    let builder = Builder::new();
    let result = builder.and_then(|_| {
        Ok(Parts {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            extensions: Extensions::new(),
            _priv: (),
        })
    });

    assert!(result.inner.is_ok());
}

#[test]
fn test_and_then_failure() {
    struct TestData;
    impl TryInto<StatusCode> for TestData {
        type Error = crate::Error;

        fn try_into(self) -> Result<StatusCode, Self::Error> {
            Err(crate::Error)
        }
    }

    let builder = Builder::new();
    let result = builder.and_then(|_| {
        Err(crate::Error)
    });

    assert!(result.inner.is_err());
}

#[test]
fn test_and_then_empty_parts() {
    struct TestData;
    impl TryInto<StatusCode> for TestData {
        type Error = crate::Error;

        fn try_into(self) -> Result<StatusCode, Self::Error> {
            Ok(StatusCode::NO_CONTENT)
        }
    }

    let builder = Builder::new();
    let result = builder.and_then(|_| {
        Ok(Parts {
            status: StatusCode::NO_CONTENT,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            extensions: Extensions::new(),
            _priv: (),
        })
    });

    assert!(result.inner.is_ok());
}

