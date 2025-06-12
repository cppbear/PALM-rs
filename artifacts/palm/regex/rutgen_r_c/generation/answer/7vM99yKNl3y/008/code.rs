// Answer 0

#[test]
fn test_from_formatter_with_valid_inputs() {
    struct DummyError;
    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Dummy error")
        }
    }
    
    let main_span = Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 5 } };
    let aux_span = Span { start: Position { line: 1, column: 6 }, end: Position { line: 1, column: 10 } };
    let formatter = Formatter {
        pattern: "abcde",
        err: &DummyError,
        span: &main_span,
        aux_span: Some(&aux_span),
    };

    let spans = Spans::from_formatter(&formatter);
    
    // Assertions on the spans
    assert_eq!(spans.pattern, "abcde");
    assert_eq!(spans.line_number_width, 0); // Since line_count is 1
    assert_eq!(spans.by_line.len(), 1);
    assert_eq!(spans.by_line[0].len(), 1); // Main span added
    assert_eq!(spans.by_line[0][0], main_span);
    assert_eq!(spans.multi_line.len(), 1); // Aux span added
    assert_eq!(spans.multi_line[0], aux_span);
}

#[test]
fn test_from_formatter_with_multiple_lines() {
    struct DummyError;
    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Dummy error")
        }
    }

    let main_span = Span { start: Position { line: 1, column: 0 }, end: Position { line: 2, column: 5 } };
    let aux_span = Span { start: Position { line: 2, column: 6 }, end: Position { line: 3, column: 10 } };
    let formatter = Formatter {
        pattern: "abcde\nfghij",
        err: &DummyError,
        span: &main_span,
        aux_span: Some(&aux_span),
    };

    let spans = Spans::from_formatter(&formatter);

    // Assertions on the spans
    assert_eq!(spans.pattern, "abcde\nfghij");
    assert_eq!(spans.line_number_width, 1); // Since line_count is 2
    assert_eq!(spans.by_line.len(), 3);
    assert_eq!(spans.by_line[0].len(), 1); // Main span added to line 1
    assert_eq!(spans.by_line[1].len(), 0); // No span in line 2
    assert_eq!(spans.by_line[2].len(), 1); // Aux span added to line 3
    assert_eq!(spans.by_line[2][0], aux_span);
    assert_eq!(spans.multi_line.len(), 0); // No multi-line spans added
}

