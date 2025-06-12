// Answer 0

#[test]
fn test_headers_empty_initialization() {
    struct Request<T> {
        head: HeaderMap<HeaderValue>,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Self {
                head: HeaderMap::new(),
                _marker: std::marker::PhantomData,
            }
        }
    }

    let request: Request<()> = Request::default();
    assert!(request.headers().is_empty());
}

#[test]
fn test_headers_non_empty_after_adding() {
    use http::header::{HeaderMap, HeaderValue};

    struct Request<T> {
        head: HeaderMap<HeaderValue>,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Self {
                head: HeaderMap::new(),
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl<T> Request<T> {
        pub fn add_header(&mut self, key: &str, value: &str) {
            self.head.insert(HeaderName::from_static(key), HeaderValue::from_static(value));
        }
    }

    let mut request: Request<()> = Request::default();
    request.add_header("Content-Type", "application/json");
    assert!(!request.headers().is_empty());
}

