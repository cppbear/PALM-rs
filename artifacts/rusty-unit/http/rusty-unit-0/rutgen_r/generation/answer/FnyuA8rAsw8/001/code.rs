// Answer 0

#[test]
fn test_headers_mut_success() {
    use http::*;
    use http::header::{HeaderValue, HeaderMap};
    
    struct Response {
        inner: Result<ResponseInner, ()>,
    }

    struct ResponseInner {
        headers: HeaderMap<HeaderValue>,
    }
    
    impl Response {
        fn builder() -> Self {
            Response {
                inner: Ok(ResponseInner {
                    headers: HeaderMap::new(),
                }),
            }
        }

        fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
            self.inner.as_mut().ok().map(|h| &mut h.headers)
        }

        fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {
            self.inner.as_ref().ok().map(|h| &h.headers)
        }
    }

    let mut res = Response::builder();
    {
        let headers = res.headers_mut().unwrap();
        headers.insert("Accept", HeaderValue::from_static("text/html"));
        headers.insert("X-Custom-Foo", HeaderValue::from_static("bar"));
    }

    let headers = res.headers_ref().unwrap();
    assert_eq!(headers["Accept"], "text/html");
    assert_eq!(headers["X-Custom-Foo"], "bar");
}

#[test]
fn test_headers_mut_failure() {
    use http::*;
    
    struct Response {
        inner: Result<ResponseInner, ()>,
    }

    struct ResponseInner {
        headers: HeaderMap<HeaderValue>,
    }

    impl Response {
        fn builder() -> Self {
            Response {
                inner: Err(()),
            }
        }

        fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
            self.inner.as_mut().ok().map(|h| &mut h.headers)
        }
    }

    let mut res = Response::builder();
    let headers_mut = res.headers_mut();

    assert!(headers_mut.is_none());
}

