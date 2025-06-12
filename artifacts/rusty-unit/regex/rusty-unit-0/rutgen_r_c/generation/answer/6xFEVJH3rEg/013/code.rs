// Answer 0

#[test]
fn test_fmt_single_line_no_error() {
    struct TestError;
    
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Test error message")
        }
    }

    let pattern = "simple regex";
    let err = TestError;
    let span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 15 } };
    let formatter = Formatter {
        pattern,
        err: &err,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = write!(&mut output, "{}", formatter);
    assert!(result.is_ok());
    let expected_output = format!(
        "regex parse error:\n{}error: {}",
        formatter.notate(),
        err
    );
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
#[should_panic]
fn test_fmt_single_line_error() {
    struct TestError;
    
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            Err(fmt::Error) // Trigger an error on display
        }
    }
    
    let pattern = "simple regex";
    let err = TestError;
    let span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 15 } };
    let formatter = Formatter {
        pattern,
        err: &err,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    let _ = write!(&mut output, "{}", formatter);
}

