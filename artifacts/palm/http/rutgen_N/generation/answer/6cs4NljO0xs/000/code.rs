// Answer 0

#[test]
fn test_fmt() {
    struct Authority {
        inner: String,
    }

    impl Authority {
        fn new(inner: &str) -> Self {
            Authority {
                inner: inner.to_string(),
            }
        }

        fn as_str(&self) -> &str {
            &self.inner
        }
    }

    use std::fmt;

    impl fmt::Display for Authority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let authority = Authority::new("example.com");
    let mut output = String::new();
    let result = write!(&mut output, "{}", authority);
    assert!(result.is_ok());
    assert_eq!(output, "example.com");
}

