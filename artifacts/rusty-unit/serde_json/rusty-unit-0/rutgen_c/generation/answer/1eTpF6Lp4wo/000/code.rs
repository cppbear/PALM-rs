// Answer 0

#[test]
fn test_fmt_with_zero_line() {
    struct TestError {
        code: crate::ErrorCode,
        line: usize,
        column: usize,
    }

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.line == 0 {
                write!(f, "{:?}", self.code)
            } else {
                write!(f, "{} at line {} column {}", self.code, self.line, self.column)
            }
        }
    }

    let error = TestError {
        code: crate::ErrorCode::Message("This is an error".into()),
        line: 0,
        column: 0,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Message(\"This is an error\")");
}

#[test]
fn test_fmt_with_non_zero_line() {
    struct TestError {
        code: crate::ErrorCode,
        line: usize,
        column: usize,
    }

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.line == 0 {
                write!(f, "{:?}", self.code)
            } else {
                write!(f, "{} at line {} column {}", self.code, self.line, self.column)
            }
        }
    }

    let error = TestError {
        code: crate::ErrorCode::EofWhileParsingObject,
        line: 5,
        column: 12,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "EofWhileParsingObject at line 5 column 12");
}

