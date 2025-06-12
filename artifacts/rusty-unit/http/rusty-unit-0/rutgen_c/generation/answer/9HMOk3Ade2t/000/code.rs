// Answer 0

#[test]
fn test_headers_mut_success() {
    struct TestBuilder {
        inner: Result<Parts>,
    }

    impl TestBuilder {
        fn new() -> Self {
            TestBuilder { inner: Ok(Parts {
                method: Method::GET,
                uri: Uri::default(),
                version: Version::default(),
                headers: HeaderMap::default(),
                extensions: Extensions::default(),
                _priv: (),
            }) }
        }

        fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
            self.inner.as_mut().ok().map(|h| &mut h.headers)
        }
    }

    let mut builder = TestBuilder::new();
    let headers = builder.headers_mut().unwrap();
    headers.entries.push(Bucket::new("Accept".to_string(), HeaderValue::from_static("text/html")));
    headers.entries.push(Bucket::new("X-Custom-Foo".to_string(), HeaderValue::from_static("bar")));
    
    let headers_ref = builder.headers_mut().unwrap();
    assert_eq!(headers_ref.entries[0].value, "text/html");
    assert_eq!(headers_ref.entries[1].value, "bar");
}

#[test]
fn test_headers_mut_failure() {
    struct TestBuilder {
        inner: Result<Parts>,
    }

    impl TestBuilder {
        fn new_with_error() -> Self {
            TestBuilder { inner: Err(Error { inner: ErrorKind::SomeError }) }
        }

        fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
            self.inner.as_mut().ok().map(|h| &mut h.headers)
        }
    }

    let mut builder = TestBuilder::new_with_error();
    let headers = builder.headers_mut();
    assert!(headers.is_none());
}

