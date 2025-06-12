// Answer 0

#[test]
fn test_error_code_display_key_must_be_a_string() {
    struct MockIoError;

    impl Debug for MockIoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Mock I/O Error")
        }
    }

    impl Display for MockIoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Mock I/O Error")
        }
    }

    let error_code = ErrorCode::KeyMustBeAString;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "key must be a string");
}

#[test]
fn test_error_code_display_expected_numeric_key() {
    struct MockIoError;

    impl Debug for MockIoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Mock I/O Error")
        }
    }

    impl Display for MockIoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Mock I/O Error")
        }
    }

    let error_code = ErrorCode::ExpectedNumericKey;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid value: expected key to be a number in quotes");
}

