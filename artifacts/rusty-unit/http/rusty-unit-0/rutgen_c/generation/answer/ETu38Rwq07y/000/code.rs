// Answer 0

#[test]
fn test_status_code_debug_fmt() {
    struct TestStatusCode(NonZeroU16);

    impl fmt::Debug for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    let value = NonZeroU16::new(200).unwrap();
    let status_code = TestStatusCode(value);
    let mut output = String::new();
    let result = write!(output, "{:?}", status_code);

    assert!(result.is_ok());
    assert_eq!(output, "200");
}

#[test]
fn test_status_code_debug_fmt_nonzero_value() {
    struct TestStatusCode(NonZeroU16);

    impl fmt::Debug for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    let value = NonZeroU16::new(404).unwrap();
    let status_code = TestStatusCode(value);
    let mut output = String::new();
    let result = write!(output, "{:?}", status_code);

    assert!(result.is_ok());
    assert_eq!(output, "404");
}

#[test]
fn test_status_code_debug_fmt_minimum_value() {
    struct TestStatusCode(NonZeroU16);

    impl fmt::Debug for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    let value = NonZeroU16::new(100).unwrap();
    let status_code = TestStatusCode(value);
    let mut output = String::new();
    let result = write!(output, "{:?}", status_code);

    assert!(result.is_ok());
    assert_eq!(output, "100");
}

