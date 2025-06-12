// Answer 0

#[test]
fn test_headers_ref_valid_headers() {
    use http::{Request, header::{HeaderMap, HeaderValue}};

    struct RequestBuilder {
        inner: Result<Request<(), HeaderMap>, &'static str>,
    }

    impl RequestBuilder {
        fn builder() -> Self {
            RequestBuilder {
                inner: Ok(Request::builder()
                    .header("Accept", "text/html")
                    .header("X-Custom-Foo", "bar")
                    .body(()).unwrap()),
            }
        }

        fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {
            self.inner.as_ref().ok().map(|h| &h.headers)
        }
    }

    let req_builder = RequestBuilder::builder();
    let headers = req_builder.headers_ref().unwrap();
    assert_eq!(headers["Accept"], "text/html");
    assert_eq!(headers["X-Custom-Foo"], "bar");
}

#[test]
fn test_headers_ref_error_case() {
    use http::{Request, header::{HeaderMap, HeaderValue}};

    struct RequestBuilder {
        inner: Result<Request<(), HeaderMap>, &'static str>,
    }

    impl RequestBuilder {
        fn builder_with_error() -> Self {
            RequestBuilder {
                inner: Err("some error"),
            }
        }

        fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {
            self.inner.as_ref().ok().map(|h| &h.headers)
        }
    }

    let req_builder = RequestBuilder::builder_with_error();
    let headers = req_builder.headers_ref();
    assert!(headers.is_none());
}

