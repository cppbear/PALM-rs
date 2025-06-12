// Answer 0

#[test]
fn test_method_ref_default() {
    struct TestBuilder {
        inner: Result<Parts>,
    }

    impl TestBuilder {
        fn new() -> Self {
            TestBuilder {
                inner: Ok(Parts {
                    method: Method(Inner),
                    uri: Uri::default(),
                    version: Version::HTTP_11,
                    headers: HeaderMap::new(),
                    extensions: Extensions::new(),
                    _priv: (),
                }),
            }
        }

        fn method_ref(&self) -> Option<&Method> {
            self.inner.as_ref().ok().map(|h| &h.method)
        }
    }

    let builder = TestBuilder::new();
    assert_eq!(builder.method_ref(), Some(&Method(Inner)));
}

#[test]
fn test_method_ref_error() {
    struct TestBuilder {
        inner: Result<Parts>,
    }

    impl TestBuilder {
        fn new_with_error() -> Self {
            TestBuilder {
                inner: Err(Error { inner: ErrorKind }),
            }
        }

        fn method_ref(&self) -> Option<&Method> {
            self.inner.as_ref().ok().map(|h| &h.method)
        }
    }

    let builder = TestBuilder::new_with_error();
    assert_eq!(builder.method_ref(), None);
}

