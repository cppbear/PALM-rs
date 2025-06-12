// Answer 0

#[test]
fn test_and_then_success() {
    struct DummyParts;
    impl TryInto<Parts> for DummyParts {
        type Error = crate::Error;

        fn try_into(self) -> Result<Parts, Self::Error> {
            Ok(Parts {
                method: Method::GET,
                uri: Uri::from_static("http://example.com"),
                version: Version::HTTP_11,
                headers: HeaderMap::new(),
                extensions: Extensions::new(),
                _priv: (),
            })
        }
    }

    let builder = Builder::new();
    let result = builder.and_then(|parts| {
        assert_eq!(parts.method, Method::GET);
        Ok(parts)
    });

    assert!(result.inner.is_ok());
}

#[test]
fn test_and_then_failure() {
    struct DummyPartsWithError;
    impl TryInto<Parts> for DummyPartsWithError {
        type Error = crate::Error;

        fn try_into(self) -> Result<Parts, Self::Error> {
            Err(crate::Error::new(crate::ErrorKind::from("dummy error")))
        }
    }

    let builder = Builder::new();
    let result = builder.and_then(|_| {
        Err(crate::Error::new(crate::ErrorKind::from("forced error")))
    });

    assert!(result.inner.is_err());
}

