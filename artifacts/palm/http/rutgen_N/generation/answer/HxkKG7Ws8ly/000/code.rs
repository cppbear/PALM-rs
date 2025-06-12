// Answer 0

#[test]
fn test_extensions_ref_with_extensions() {
    struct Inner {
        extensions: Extensions,
    }

    struct Response {
        inner: Result<Inner, ()>,
    }

    impl Response {
        fn builder() -> ResponseBuilder {
            ResponseBuilder::new()
        }
    }

    struct ResponseBuilder {
        inner: Result<Inner, ()>,
    }

    impl ResponseBuilder {
        fn new() -> Self {
            Self { inner: Ok(Inner { extensions: Extensions::new() }) }
        }

        fn extension<T: 'static>(&mut self, value: T) -> &mut Self {
            self.inner.as_mut().ok().map(|inner| {
                inner.extensions.insert(value);
            });
            self
        }

        fn build(self) -> Response {
            Response { inner: self.inner }
        }
    }

    struct Extensions {
        data: std::collections::HashMap<&'static str, Box<dyn std::any::Any>>,
    }

    impl Extensions {
        fn new() -> Self {
            Self { data: std::collections::HashMap::new() }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            self.data.insert(std::any::type_name::<T>(), Box::new(value));
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.data.get(std::any::type_name::<T>())?.downcast_ref::<T>()
        }
    }

    let mut res = Response::builder().extension("My Extension").extension(5u32).build();
    let extensions = res.extensions_ref().unwrap();
    assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    assert_eq!(extensions.get::<u32>(), Some(&5u32));
}

#[test]
fn test_extensions_ref_with_no_error() {
    struct Inner {
        extensions: Extensions,
    }

    struct Response {
        inner: Result<Inner, ()>,
    }

    impl Response {
        fn builder() -> ResponseBuilder {
            ResponseBuilder::new()
        }
    }

    struct ResponseBuilder {
        inner: Result<Inner, ()>,
    }

    impl ResponseBuilder {
        fn new() -> Self {
            Self { inner: Ok(Inner { extensions: Extensions::new() }) }
        }

        fn build(self) -> Response {
            Response { inner: self.inner }
        }
    }

    struct Extensions {
        data: std::collections::HashMap<&'static str, Box<dyn std::any::Any>>,
    }

    impl Extensions {
        fn new() -> Self {
            Self { data: std::collections::HashMap::new() }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.data.get(std::any::type_name::<T>())?.downcast_ref::<T>()
        }
    }

    let res = Response::builder().build();
    let extensions = res.extensions_ref();
    assert_eq!(extensions, Some(&Extensions::new()));
}

