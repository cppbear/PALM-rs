// Answer 0

#[test]
fn test_notate_with_line_number_width_zero_and_no_spans() {
    struct DummySpan { start: Position, end: Position }
    
    let pattern = "abc\ndef\nghi"; // Multi-line pattern
    let by_line: Vec<Vec<DummySpan>> = vec![vec![], vec![], vec![]]; // No spans
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let expected = "    abc\n    def\n    ghi\n";
    let result = spans.notate();
    assert_eq!(result, expected);
}

#[test]
fn test_notate_with_single_line_with_spans() {
    struct DummySpan { start: Position, end: Position }

    let pattern = "abc"; // Single-line pattern
    let by_line: Vec<Vec<DummySpan>> = vec![vec![DummySpan { start: Position::new(1, 1), end: Position::new(1, 2) }]]; // One span
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 1,
        by_line,
        multi_line,
    };

    let expected = " 1: abc\n  ^\n"; // Line number padding and caret notation
    let result = spans.notate();
    assert_eq!(result, expected);
}

#[test]
fn test_notate_with_no_spans_on_multiple_lines() {
    struct DummySpan { start: Position, end: Position }

    let pattern = "line1\nline2\nline3"; // Multi-line pattern
    let by_line: Vec<Vec<DummySpan>> = vec![vec![], vec![], vec![]]; // No spans
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 2,
        by_line,
        multi_line,
    };

    let expected = " 1: line1\n 2: line2\n 3: line3\n"; // Line numbers with padding
    let result = spans.notate();
    assert_eq!(result, expected);
}

#[test]
fn test_notate_with_spans_on_multiple_lines() {
    struct DummySpan { start: Position, end: Position }

    let pattern = "line1\nline2\nline3"; // Multi-line pattern
    let by_line: Vec<Vec<DummySpan>> = vec![
        vec![DummySpan { start: Position::new(1, 2), end: Position::new(1, 3) }], // Span in line 1
        vec![], // No spans in line 2
        vec![DummySpan { start: Position::new(3, 1), end: Position::new(3, 2) }], // Span in line 3
    ];
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 2,
        by_line,
        multi_line,
    };

    let expected = " 1: line1\n  ^\n 2: line2\n 3: line3\n  ^\n"; // Line numbers with spans indicated
    let result = spans.notate();
    assert_eq!(result, expected);
}

