// Answer 0

#[test]
fn test_formatter_single_line_error() {
    struct MockError;
    
    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Mock error message")
        }
    }

    let pattern = "single_line_pattern";
    let error = MockError;
    let span = ast::Span { start: Position::new(0, 0), end: Position::new(0, 18) };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = write!(&mut output, "{}", formatter);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "regex parse error:\nerror: Mock error message");
}

#[test]
fn test_formatter_single_line_error_with_no_error() {
    struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "")
        }
    }

    let pattern = "another_single_line_pattern";
    let error = MockError;
    let span = ast::Span { start: Position::new(0, 0), end: Position::new(0, 25) };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = write!(&mut output, "{}", formatter);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "regex parse error:\nerror: ");
}

