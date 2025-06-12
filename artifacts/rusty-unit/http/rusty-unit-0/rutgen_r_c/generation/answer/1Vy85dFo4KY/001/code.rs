// Answer 0

#[test]
fn test_from_name_valid() {
    #[derive(Clone)]
    struct TestHeaderName {
        inner: Bytes,
    }

    impl From<TestHeaderName> for HeaderValue {
        fn from(name: TestHeaderName) -> HeaderValue {
            HeaderValue {
                inner: name.inner,
                is_sensitive: false,
            }
        }
    }

    // Valid HeaderName example
    let name = TestHeaderName {
        inner: Bytes::from_static(b"accept"),
    };
    let val = HeaderValue::from_name(name.clone());
    assert_eq!(val, HeaderValue::from_bytes(b"accept").unwrap());
}

#[test]
fn test_from_name_empty() {
    #[derive(Clone)]
    struct TestHeaderName {
        inner: Bytes,
    }

    impl From<TestHeaderName> for HeaderValue {
        fn from(name: TestHeaderName) -> HeaderValue {
            HeaderValue {
                inner: name.inner,
                is_sensitive: false,
            }
        }
    }

    // Test empty HeaderName
    let name = TestHeaderName {
        inner: Bytes::from_static(b""),
    };
    let val = HeaderValue::from_name(name.clone());
    assert_eq!(val, HeaderValue::from_bytes(b"").unwrap());
}

#[test]
#[should_panic]
fn test_from_name_invalid() {
    // Create a mock invalid HeaderName scenario
    #[derive(Clone)]
    struct InvalidHeaderName {
        inner: Bytes,
    }

    impl From<InvalidHeaderName> for HeaderValue {
        fn from(name: InvalidHeaderName) -> HeaderValue {
            panic!("Invalid Header Name");
        }
    }

    let name = InvalidHeaderName {
        inner: Bytes::from_static(b"invalid"),
    };
    HeaderValue::from_name(name);
}

