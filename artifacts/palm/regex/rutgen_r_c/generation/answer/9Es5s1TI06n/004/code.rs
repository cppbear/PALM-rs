// Answer 0

#[test]
fn test_repetition_missing() {
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.kind)
        }
    }

    let error = TestError {
        kind: ErrorKind::RepetitionMissing,
    };

    let mut output = String::new();
    let result = fmt(&error, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, "repetition operator missing expression");
}

#[test]
fn test_repetition_missing_with_large_nest_limit() {
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.kind)
        }
    }

    let error = TestError {
        kind: ErrorKind::NestLimitExceeded(100),
    };

    let mut output = String::new();
    let result = fmt(&error, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, "exceed the maximum number of \
                       nested parentheses/brackets (100)");
}

