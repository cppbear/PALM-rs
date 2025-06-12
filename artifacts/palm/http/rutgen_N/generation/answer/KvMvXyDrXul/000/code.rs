// Answer 0

#[test]
fn test_extensions_mut_on_valid_request() {
    use http::{Request, Extensions};

    struct TestRequest {
        inner: Option<Request<'static, ()>>, // assuming that Request uses some type in the generic parameter
    }

    impl TestRequest {
        fn new() -> Self {
            Self {
                inner: Some(Request::builder().body(()).unwrap()), // Initialize with a valid request
            }
        }

        fn extensions_mut(&mut self) -> Option<&mut Extensions> {
            self.inner.as_mut().ok().map(|h| &mut h.extensions)
        }
    }

    let mut req = TestRequest::new();
    req.extensions_mut().unwrap().insert("My Extension");
    let extensions = req.extensions_mut().unwrap();
    assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
}

#[test]
fn test_extensions_mut_on_error_request() {
    use http::{Request, Extensions};

    struct TestRequest {
        inner: Option<Request<'static, ()>>, // assuming that Request uses some type in the generic parameter
    }

    impl TestRequest {
        fn new() -> Self {
            Self {
                inner: None, // Initialize with an error state (None)
            }
        }

        fn extensions_mut(&mut self) -> Option<&mut Extensions> {
            self.inner.as_mut().ok().map(|h| &mut h.extensions)
        }
    }

    let mut req = TestRequest::new();
    assert!(req.extensions_mut().is_none());
}

