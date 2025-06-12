// Answer 0

#[test]
fn test_header_with_valid_key_value() {
    struct ValidKey;
    struct ValidValue;

    impl TryInto<HeaderName> for ValidKey {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderName, Self::Error> {
            Ok(HeaderName::from_static("X-Custom-Header"))
        }
    }

    impl TryInto<HeaderValue> for ValidValue {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderValue, Self::Error> {
            Ok(HeaderValue::from_static("HeaderValue"))
        }
    }

    let builder = Builder::new();
    let builder = builder.header(ValidKey, ValidValue);
    assert!(builder.inner.is_ok());
}

#[test]
fn test_header_with_invalid_key() {
    struct InvalidKey;

    impl TryInto<HeaderName> for InvalidKey {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderName, Self::Error> {
            Err(crate::Error::from("Invalid header name"))
        }
    }

    struct ValidValue;

    impl TryInto<HeaderValue> for ValidValue {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderValue, Self::Error> {
            Ok(HeaderValue::from_static("HeaderValue"))
        }
    }

    let builder = Builder::new();
    let builder = builder.header(InvalidKey, ValidValue);
    assert!(builder.inner.is_err());
}

#[test]
fn test_header_with_invalid_value() {
    struct ValidKey;

    impl TryInto<HeaderName> for ValidKey {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderName, Self::Error> {
            Ok(HeaderName::from_static("X-Custom-Header"))
        }
    }

    struct InvalidValue;

    impl TryInto<HeaderValue> for InvalidValue {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderValue, Self::Error> {
            Err(crate::Error::from("Invalid header value"))
        }
    }

    let builder = Builder::new();
    let builder = builder.header(ValidKey, InvalidValue);
    assert!(builder.inner.is_err());
}

#[test]
fn test_header_with_multiple_calls() {
    struct ValidKey1;
    struct ValidKey2;
    struct ValidValue;

    impl TryInto<HeaderName> for ValidKey1 {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderName, Self::Error> {
            Ok(HeaderName::from_static("X-Custom-Header-1"))
        }
    }

    impl TryInto<HeaderName> for ValidKey2 {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderName, Self::Error> {
            Ok(HeaderName::from_static("X-Custom-Header-2"))
        }
    }

    impl TryInto<HeaderValue> for ValidValue {
        type Error = crate::Error;
        fn try_into(self) -> Result<HeaderValue, Self::Error> {
            Ok(HeaderValue::from_static("HeaderValue"))
        }
    }

    let builder = Builder::new();
    let builder = builder.header(ValidKey1, ValidValue)
                       .header(ValidKey2, ValidValue);
    assert!(builder.inner.is_ok());
}

