// Answer 0

#[test]
fn test_fmt() {
    struct TestScheme {
        value: &'static str,
    }

    impl TestScheme {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    use std::fmt;

    impl fmt::Display for TestScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let scheme = TestScheme { value: "http" };
    let mut output = String::new();
    let result = write!(&mut output, "{}", scheme);

    assert!(result.is_ok());
    assert_eq!(output, "http");
}

#[test]
fn test_fmt_empty() {
    struct TestScheme {
        value: &'static str,
    }

    impl TestScheme {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    use std::fmt;

    impl fmt::Display for TestScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.as_str())
        }
    }

    let scheme = TestScheme { value: "" };
    let mut output = String::new();
    let result = write!(&mut output, "{}", scheme);

    assert!(result.is_ok());
    assert_eq!(output, "");
}

