// Answer 0

#[test]
fn test_extension_inserts_extension() {
    use std::any::Any;

    struct TestBuilder {
        inner: Result<Parts>,
    }

    impl TestBuilder {
        fn new() -> Self {
            TestBuilder {
                inner: Ok(Parts {
                    status: StatusCode::OK,
                    version: Version::default(),
                    headers: HeaderMap::new(),
                    extensions: Extensions::new(),
                    _priv: (),
                }),
            }
        }

        fn extension<T>(self, extension: T) -> TestBuilder
        where
            T: Clone + Any + Send + Sync + 'static,
        {
            self.and_then(move |mut head| {
                head.extensions.insert(extension);
                Ok(head)
            })
        }

        fn and_then<F>(self, func: F) -> Self
        where
            F: FnOnce(Parts) -> Result<Parts>,
        {
            TestBuilder {
                inner: self.inner.and_then(func),
            }
        }

        fn extensions(self) -> Extensions {
            // Assuming inner is Ok and contains a Parts instance
            self.inner.unwrap().extensions
        }
    }

    let response = TestBuilder::new()
        .extension("My Extension")
        .body(())
        .unwrap();

    assert_eq!(response.extensions().get::<&'static str>(), Some(&"My Extension"));
}

#[test]
fn test_extension_with_different_data_type() {
    use std::any::Any;

    struct TestBuilder {
        inner: Result<Parts>,
    }

    impl TestBuilder {
        fn new() -> Self {
            TestBuilder {
                inner: Ok(Parts {
                    status: StatusCode::OK,
                    version: Version::default(),
                    headers: HeaderMap::new(),
                    extensions: Extensions::new(),
                    _priv: (),
                }),
            }
        }

        fn extension<T>(self, extension: T) -> TestBuilder
        where
            T: Clone + Any + Send + Sync + 'static,
        {
            self.and_then(move |mut head| {
                head.extensions.insert(extension);
                Ok(head)
            })
        }

        fn and_then<F>(self, func: F) -> Self
        where
            F: FnOnce(Parts) -> Result<Parts>,
        {
            TestBuilder {
                inner: self.inner.and_then(func),
            }
        }

        fn extensions(self) -> Extensions {
            self.inner.unwrap().extensions
        }
    }

    let response = TestBuilder::new()
        .extension(42_i32)
        .body(())
        .unwrap();

    assert_eq!(response.extensions().get::<i32>(), Some(&42));
}

