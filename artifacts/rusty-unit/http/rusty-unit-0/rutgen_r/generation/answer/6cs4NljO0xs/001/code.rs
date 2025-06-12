// Answer 0

#[test]
fn test_fmt_with_valid_string() {
    struct Authority {
        inner: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.inner
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let authority = Authority {
        inner: String::from("http://example.com"),
    };
    let mut output = String::new();
    let result = authority.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "http://example.com");
}

#[test]
#[should_panic]
fn test_fmt_with_empty_string() {
    struct Authority {
        inner: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.inner
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let authority = Authority {
        inner: String::new(),
    };
    let mut output = String::new();
    authority.fmt(&mut std::fmt::Formatter::new(&mut output)).unwrap();
}

#[test]
fn test_fmt_with_special_characters() {
    struct Authority {
        inner: String,
    }

    impl Authority {
        fn as_str(&self) -> &str {
            &self.inner
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let authority = Authority {
        inner: String::from("http://example.com/path?query=value#fragment"),
    };
    let mut output = String::new();
    let result = authority.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "http://example.com/path?query=value#fragment");
}

