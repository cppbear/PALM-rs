// Answer 0

#[test]
fn test_request_builder_initialization() {
    let builder = Request::builder();
    assert_eq!(builder.inner.is_ok(), true);
}

#[test]
fn test_request_builder_method() {
    struct TestMethod;
    impl TryInto<Method> for TestMethod {
        type Error = crate::Error;

        fn try_into(self) -> Result<Method> {
            Ok(Method::from_bytes(b"GET").unwrap())
        }
    }

    let builder = Request::builder().method(TestMethod);
    assert!(builder.inner.is_ok());
}

#[test]
fn test_request_builder_uri() {
    struct TestUri;
    impl TryInto<Uri> for TestUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri::from_static("https://www.rust-lang.org/"))
        }
    }

    let builder = Request::builder().uri(TestUri);
    assert!(builder.inner.is_ok());
}

#[test]
fn test_request_builder_header() {
    struct TestHeaderName;
    impl TryInto<HeaderName> for TestHeaderName {
        type Error = crate::Error;

        fn try_into(self) -> Result<HeaderName> {
            Ok(HeaderName::from_static("X-Custom-Foo"))
        }
    }

    struct TestHeaderValue;
    impl TryInto<HeaderValue> for TestHeaderValue {
        type Error = crate::Error;

        fn try_into(self) -> Result<HeaderValue> {
            Ok(HeaderValue::from_static("Bar"))
        }
    }

    let builder = Request::builder()
        .header(TestHeaderName, TestHeaderValue);
    assert!(builder.inner.is_ok());
}

