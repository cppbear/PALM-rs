// Answer 0

#[test]
fn test_formatter_with_line_breaks() {
    struct DummyError;
    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "dummy error")
        }
    }

    let pattern = "abc\ndef\n";
    let span1 = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 3 } };
    let span2 = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 3 } };
    let multi_line_span = ast::Span { start: Position { line: 0, column: 3 }, end: Position { line: 1, column: 0 } };

    let spans = Spans {
        pattern,
        line_number_width: 1,
        by_line: vec![vec![span1], vec![span2]],
        multi_line: vec![multi_line_span],
    };
    
    let formatter = Formatter {
        pattern,
        err: &DummyError,
        span: &span1,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = formatter.fmt(&mut output);
}

