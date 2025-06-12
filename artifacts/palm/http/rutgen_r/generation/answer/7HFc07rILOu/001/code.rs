// Answer 0

#[test]
fn test_fmt_with_valid_scheme() {
    struct Scheme {
        value: &'static str,
    }

    impl Scheme {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let scheme = Scheme { value: "http" };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| scheme.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "http");
}

#[test]
fn test_fmt_with_empty_scheme() {
    struct Scheme {
        value: &'static str,
    }

    impl Scheme {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let scheme = Scheme { value: "" };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| scheme.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
#[should_panic]
fn test_fmt_with_null_scheme() {
    struct Scheme {
        value: Option<&'static str>,
    }

    impl Scheme {
        fn as_str(&self) -> &str {
            self.value.unwrap_or_else(|| panic!("Scheme value is null"))
        }
    }

    let scheme = Scheme { value: None };
    let mut output = String::new();
    let _ = std::fmt::write(&mut output, |f| scheme.fmt(f));
}

