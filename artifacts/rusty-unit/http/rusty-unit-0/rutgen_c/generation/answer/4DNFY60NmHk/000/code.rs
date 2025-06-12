// Answer 0

#[test]
fn test_response_builder_creation() {
    let builder = Response::<()>::builder();
    assert!(builder.inner.is_ok());
}

#[test]
fn test_builder_new() {
    let builder = Builder::new();
    assert!(builder.inner.is_ok());
}

#[test]
fn test_builder_set_status() {
    struct TestStatus(u16);
    impl TryInto<StatusCode> for TestStatus {
        type Error = ();

        fn try_into(self) -> Result<StatusCode, Self::Error> {
            Ok(StatusCode::from_u16(self.0).unwrap())
        }
    }

    let builder = Response::<()>::builder().status(TestStatus(200));
    assert!(builder.inner.is_ok());
}

#[test]
fn test_builder_set_version() {
    let version = Version::HTTP_11;
    let builder = Response::<()>::builder().version(version);
    assert!(builder.inner.is_ok());
}

#[test]
fn test_builder_set_header() {
    struct TestHeaderName(String);
    impl TryInto<HeaderName> for TestHeaderName {
        type Error = ();

        fn try_into(self) -> Result<HeaderName, Self::Error> {
            Ok(HeaderName::from_bytes(self.0.as_bytes()).unwrap())
        }
    }

    struct TestHeaderValue(String);
    impl TryInto<HeaderValue> for TestHeaderValue {
        type Error = ();

        fn try_into(self) -> Result<HeaderValue, Self::Error> {
            Ok(HeaderValue::from_str(&self.0).unwrap())
        }
    }

    let builder = Response::<()>::builder()
        .header(TestHeaderName("X-Custom-Foo".to_string()), TestHeaderValue("Bar".to_string()));
    assert!(builder.inner.is_ok());
}

