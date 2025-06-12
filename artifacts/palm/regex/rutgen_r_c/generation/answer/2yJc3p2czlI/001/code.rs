// Answer 0

#[test]
fn test_fmt_display_error() {
    use std::fmt;
    use std::assert_eq;

    #[derive(Debug)]
    struct TestError {
        kind: ErrorKind,
        pattern: String,
        span: Span,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error kind: {:?}, Pattern: {}, Span: {:?}", self.kind, self.pattern, self.span)
        }
    }

    let pattern = "abc";
    let span = Span { start: 0, end: 3 };
    let error = TestError {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: pattern.to_string(),
        span,
    };
    
    let expected_output = format!("Error kind: {:?}, Pattern: {}, Span: {:?}", error.kind, error.pattern, error.span);
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert_eq!(result.is_ok(), true);
    assert_eq!(output, expected_output);
}

#[test]
fn test_fmt_display_error_with_empty_pattern() {
    use std::fmt;

    #[derive(Debug)]
    struct TestError {
        kind: ErrorKind,
        pattern: String,
        span: Span,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error kind: {:?}, Pattern: {}, Span: {:?}", self.kind, self.pattern, self.span)
        }
    }

    let span = Span { start: 0, end: 0 };
    let error = TestError {
        kind: ErrorKind::InvalidUtf8,
        pattern: "".to_string(),
        span,
    };

    let expected_output = format!("Error kind: {:?}, Pattern: {}, Span: {:?}", error.kind, error.pattern, error.span);
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert_eq!(result.is_ok(), true);
    assert_eq!(output, expected_output);
}

#[test]
fn test_fmt_display_error_with_large_span() {
    use std::fmt;

    #[derive(Debug)]
    struct TestError {
        kind: ErrorKind,
        pattern: String,
        span: Span,
    }
    
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error kind: {:?}, Pattern: {}, Span: {:?}", self.kind, self.pattern, self.span)
        }
    }

    let pattern = ".*";
    let span = Span { start: 0, end: u32::MAX }; // Large span
    let error = TestError {
        kind: ErrorKind::ClassUnclosed,
        pattern: pattern.to_string(),
        span,
    };

    let expected_output = format!("Error kind: {:?}, Pattern: {}, Span: {:?}", error.kind, error.pattern, error.span);
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert_eq!(result.is_ok(), true);
    assert_eq!(output, expected_output);
}

