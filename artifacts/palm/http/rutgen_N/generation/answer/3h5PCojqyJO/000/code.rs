// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Authority {
        value: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.value
        }
    }

    impl fmt::Display for Authority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let authority = Authority {
        value: String::from("localhost:8080"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", authority);

    assert!(result.is_ok());
    assert_eq!(output, "localhost:8080");
}

#[test]
fn test_fmt_empty() {
    use std::fmt;

    struct Authority {
        value: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.value
        }
    }

    impl fmt::Display for Authority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let authority = Authority {
        value: String::from(""),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", authority);

    assert!(result.is_ok());
    assert_eq!(output, "");
}

