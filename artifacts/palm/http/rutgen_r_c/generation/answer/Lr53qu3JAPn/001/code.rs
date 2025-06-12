// Answer 0

#[test]
fn test_version_ref_success_http11() {
    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            MockBuilder {
                inner: Ok(Parts {
                    version: Version::HTTP_11,
                    ..Default::default()
                }),
            }
        }
        
        fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let builder = MockBuilder::new();
    assert_eq!(builder.version_ref().unwrap(), &Version::HTTP_11);
}

#[test]
fn test_version_ref_success_http2() {
    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            MockBuilder {
                inner: Ok(Parts {
                    version: Version::HTTP_2,
                    ..Default::default()
                }),
            }
        }
        
        fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let builder = MockBuilder::new();
    assert_eq!(builder.version_ref().unwrap(), &Version::HTTP_2);
}

#[test]
fn test_version_ref_fail() {
    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            MockBuilder {
                inner: Err(Error { inner: ErrorKind::Other }),
            }
        }

        fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let builder = MockBuilder::new();
    assert!(builder.version_ref().is_none());
}

