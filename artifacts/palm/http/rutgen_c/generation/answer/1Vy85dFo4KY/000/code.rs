// Answer 0

#[test]
fn test_from_name() {
    struct TestHeaderName {
        inner: HeaderName,
    }

    impl From<&str> for TestHeaderName {
        fn from(s: &str) -> Self {
            TestHeaderName {
                inner: HeaderName {
                    inner: Repr::new(s),
                },
            }
        }
    }

    let name = TestHeaderName::from("accept");
    let val = HeaderValue::from_name(name.inner.clone());
    assert_eq!(val, HeaderValue::from_bytes(b"accept").unwrap());
}

#[test]
fn test_from_name_empty() {
    struct TestHeaderName {
        inner: HeaderName,
    }

    impl From<&str> for TestHeaderName {
        fn from(s: &str) -> Self {
            TestHeaderName {
                inner: HeaderName {
                    inner: Repr::new(s),
                },
            }
        }
    }

    let name = TestHeaderName::from("");
    let val = HeaderValue::from_name(name.inner.clone());
    assert_eq!(val.len(), 0);
    assert!(val.is_empty());
}

