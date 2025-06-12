// Answer 0

#[test]
fn test_fmt_with_multiline_error() {
    struct TestError;
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Test error")
        }
    }

    let pattern = "abc\ndef\nxyz";
    let span1 = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 3 } };
    let span2 = ast::Span { start: Position { line: 2, column: 0 }, end: Position { line: 2, column: 3 } };
    let spans = Spans {
        pattern,
        line_number_width: 2,
        by_line: vec![vec![span1], vec![span2]],
        multi_line: vec![span1.clone(), span2.clone()],
    };
    
    let formatter = Formatter {
        pattern: pattern,
        err: &TestError,
        span: &span1,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = formatter.fmt(&mut output);
    assert!(result.is_ok());

    let expected_output = format!(
        "regex parse error:\n{}\n{}\n{}\nerror: Test error",
        repeat_char('~', 79),
        spans.notate(),
        repeat_char('~', 79)
    );
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_fmt_with_no_multiline_error() {
    struct TestError;
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Test error")
        }
    }

    let pattern = "abc\ndef";
    let span1 = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 3 } };
    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line: vec![vec![span1]],
        multi_line: vec![],
    };
    
    let formatter = Formatter {
        pattern: pattern,
        err: &TestError,
        span: &span1,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = formatter.fmt(&mut output);
    assert!(result.is_ok());

    let expected_output = format!(
        "regex parse error:\n{}\nerror: Test error",
        spans.notate()
    );
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

