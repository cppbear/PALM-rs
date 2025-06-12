// Answer 0

#[test]
fn test_notate_line_with_multiple_spans() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    let start1 = Position { column: 1 };
    let end1 = Position { column: 3 };
    let span1 = DummySpan { start: start1, end: end1 };

    let start2 = Position { column: 5 };
    let end2 = Position { column: 7 };
    let span2 = DummySpan { start: start2, end: end2 };

    let spans_vec = vec![span1, span2];
    let by_line: Vec<Vec<DummySpan>> = vec![spans_vec];

    let spans = Spans {
        pattern: "a b c",
        line_number_width: 2,
        by_line,
        multi_line: vec![],
    };

    let result = spans.notate_line(0).unwrap();
    assert_eq!(result, "  ^^^^");
}

#[test]
fn test_notate_line_with_single_span() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    let start = Position { column: 2 };
    let end = Position { column: 5 };
    let span = DummySpan { start, end };

    let spans_vec = vec![span];
    let by_line: Vec<Vec<DummySpan>> = vec![spans_vec];

    let spans = Spans {
        pattern: "x y z",
        line_number_width: 2,
        by_line,
        multi_line: vec![],
    };

    let result = spans.notate_line(0).unwrap();
    assert_eq!(result, "  ^^^");
}

#[test]
fn test_notate_line_with_no_spans() {
    let by_line: Vec<Vec<ast::Span>> = vec![vec![]];

    let spans = Spans {
        pattern: "no spans here",
        line_number_width: 2,
        by_line,
        multi_line: vec![],
    };

    let result = spans.notate_line(0);
    assert!(result.is_none());
}

#[test]
fn test_notate_line_with_empty_pattern() {
    let by_line: Vec<Vec<ast::Span>> = vec![vec![]];

    let spans = Spans {
        pattern: "",
        line_number_width: 2,
        by_line,
        multi_line: vec![],
    };

    let result = spans.notate_line(0);
    assert!(result.is_none());
}

