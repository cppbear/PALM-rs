// Answer 0

#[test]
fn test_headers_mut_success() {
    use http::{header::HeaderValue, Request, HeaderMap};

    struct TestRequest {
        inner: Result<HeaderMap<HeaderValue>, ()>,
    }

    impl TestRequest {
        pub fn builder() -> Self {
            TestRequest {
                inner: Ok(HeaderMap::new()),
            }
        }

        pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
            self.inner.as_mut().ok().map(|h| &mut h)
        }
    }

    let mut req = TestRequest::builder();
    {
        let headers = req.headers_mut().unwrap();
        headers.insert("Accept", HeaderValue::from_static("text/html"));
        headers.insert("X-Custom-Foo", HeaderValue::from_static("bar"));
    }
    let headers = req.headers_mut().unwrap();
    assert_eq!(headers["Accept"], "text/html");
    assert_eq!(headers["X-Custom-Foo"], "bar");
}

#[test]
fn test_headers_mut_error() {
    use http::{header::HeaderValue, Request, HeaderMap};

    struct TestRequest {
        inner: Result<HeaderMap<HeaderValue>, ()>,
    }

    impl TestRequest {
        pub fn builder_error() -> Self {
            TestRequest {
                inner: Err(()),
            }
        }

        pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
            self.inner.as_mut().ok().map(|h| &mut h)
        }
    }

    let mut req = TestRequest::builder_error();
    let headers = req.headers_mut();
    assert!(headers.is_none());
}

