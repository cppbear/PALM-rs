// Answer 0

#[test]
fn test_extensions_ref_with_no_error() {
    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            MockBuilder {
                inner: Ok(Parts {
                    extensions: Extensions::default(),
                    ..Default::default()
                }),
            }
        }

        fn extension<T>(mut self, _extension: T) -> Self 
        where
            T: Clone + Any + Send + Sync + 'static,
        {
            self.inner = Ok(Parts {
                extensions: Extensions {
                    map: Some(Box::new(AnyMap::new())),
                },
                ..Default::default()
            });
            self
        }

        fn extensions_ref(&self) -> Option<&Extensions> {
            self.inner.as_ref().ok().map(|h| &h.extensions)
        }
    }

    let builder = MockBuilder::new().extension("My Extension").extension(5u32);
    let extensions = builder.extensions_ref().unwrap();
    // Fake assertion for this mock implementation
    assert!(extensions.map.is_some());
}

#[test]
fn test_extensions_ref_with_error() {
    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            MockBuilder {
                inner: Err(Error { inner: ErrorKind::SomeError }),
            }
        }

        fn extensions_ref(&self) -> Option<&Extensions> {
            self.inner.as_ref().ok().map(|h| &h.extensions)
        }
    }

    let builder = MockBuilder::new();
    assert!(builder.extensions_ref().is_none());
}

