// Answer 0

#[test]
fn test_formatter_display_multiline_error() {
    use std::fmt;
    
    struct MockError {
        message: &'static str,
    }
    
    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }
    
    let pattern = "a(bc\n(de";
    let error = MockError { message: "syntax error" };
    let span = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 2, column: 2 } };
    let multi_line_span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 2, column: 0 } };
    
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };

    let mut output = String::new();
    let result = formatter.fmt(&mut output);
    
    assert!(result.is_ok());
    assert!(output.contains("regex parse error:"));
    assert!(output.contains("~~~~~~~~~~~~~~~~~~~~~~~~~~~"));
    assert!(output.contains("error: syntax error"));
}

