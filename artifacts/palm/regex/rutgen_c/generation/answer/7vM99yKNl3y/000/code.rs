// Answer 0

#[test]
fn test_from_formatter_single_line() {
    struct MockSpan {
        start: Position,
        end: Position,
    }

    let span = MockSpan { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 1 } };
    let formatter = Formatter {
        pattern: "a",
        err: &"Error",
        span: &span,
        aux_span: None,
    };

    let spans = Spans::from_formatter(&formatter);
    
    assert_eq!(spans.pattern, "a");
    assert_eq!(spans.line_number_width, 0);
    assert_eq!(spans.by_line.len(), 1);
    assert_eq!(spans.by_line[0].len(), 1);
    assert_eq!(spans.multi_line.len(), 0);
}

#[test]
fn test_from_formatter_multi_line() {
    struct MockSpan {
        start: Position,
        end: Position,
    }

    let span1 = MockSpan { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 3 } };
    let span2 = MockSpan { start: Position { line: 2, column: 0 }, end: Position { line: 2, column: 2 } };
    let formatter = Formatter {
        pattern: "ab\ncd",
        err: &"Error",
        span: &span1,
        aux_span: Some(&span2),
    };

    let spans = Spans::from_formatter(&formatter);
    
    assert_eq!(spans.pattern, "ab\ncd");
    assert_eq!(spans.line_number_width, 1);
    assert_eq!(spans.by_line.len(), 2);
    assert_eq!(spans.by_line[0].len(), 1);
    assert_eq!(spans.by_line[1].len(), 1);
    assert_eq!(spans.multi_line.len(), 0);
}

#[test]
fn test_from_formatter_ends_with_newline() {
    struct MockSpan {
        start: Position,
        end: Position,
    }

    let span = MockSpan { start: Position { line: 2, column: 1 }, end: Position { line: 2, column: 2 } };
    let formatter = Formatter {
        pattern: "abc\nxyz\n",  // pattern ends with a newline
        err: &"Error",
        span: &span,
        aux_span: None,
    };

    let spans = Spans::from_formatter(&formatter);
    
    assert_eq!(spans.pattern, "abc\nxyz\n");
    assert_eq!(spans.line_number_width, 1);
    assert_eq!(spans.by_line.len(), 3);
    assert_eq!(spans.by_line[0].len(), 0);
    assert_eq!(spans.by_line[1].len(), 1); // spans for second line
    assert_eq!(spans.multi_line.len(), 0);
}

