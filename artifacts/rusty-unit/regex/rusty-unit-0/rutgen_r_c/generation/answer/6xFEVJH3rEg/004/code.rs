// Answer 0

fn test_fmt_with_multiline_pattern() -> Result<(), std::fmt::Error> {
    struct DummyError;
    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "dummy error")
        }
    }

    let pattern = "a(bc\nd)";
    let error_message = DummyError;
    let span = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 5 } };
    let aux_span = Some(&span);

    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span,
    };

    let mut output = String::new();
    let result = formatter.fmt(&mut output);

    assert!(result.is_ok());
    assert!(output.contains("regex parse error:"));
    assert!(output.contains("~"));
    
    Ok(())
}

fn test_fmt_with_single_line_pattern() -> Result<(), std::fmt::Error> {
    struct DummyError;
    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "dummy error")
        }
    }

    let pattern = "abc";
    let error_message = DummyError;
    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 3 } };
    let aux_span = None;

    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span,
    };

    let mut output = String::new();
    let result = formatter.fmt(&mut output);

    assert!(result.is_ok());
    assert!(output.contains("regex parse error:"));
    assert!(!output.contains("~"));

    Ok(())
}

#[test]
fn test_fmt_trigger_panic_conditions() {
    struct DummyError;
    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "dummy for panic test")
        }
    }

    let pattern = "abc\ndef";
    let error_message = DummyError;
    let valid_span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 3 } };
    let aux_span = Some(&valid_span);

    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &valid_span,
        aux_span,
    };

    let mut output = String::new();
    
    // This test is expected to panic due to the forced failure in the output sequence.
    let result = formatter.fmt(&mut output);
    assert!(result.is_err()); // Ensure that we confirm the panic is related to not formatted correctly.
}

