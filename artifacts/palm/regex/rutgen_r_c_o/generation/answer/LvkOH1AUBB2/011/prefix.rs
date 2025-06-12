// Answer 0

#[test]
fn test_notate_line_single_span_no_padding() {
    let spans = Spans {
        pattern: "abc",
        line_number_width: 0,
        by_line: vec![vec![
            ast::Span { start: Position { column: 2 }, end: Position { column: 3 } }
        ]],
        multi_line: vec![],
    };
    let result = spans.notate_line(0);
}

#[test]
fn test_notate_line_multiple_spans_no_padding() {
    let spans = Spans {
        pattern: "abcd",
        line_number_width: 0,
        by_line: vec![vec![
            ast::Span { start: Position { column: 2 }, end: Position { column: 3 } },
            ast::Span { start: Position { column: 4 }, end: Position { column: 5 } }
        ]],
        multi_line: vec![],
    };
    let result = spans.notate_line(0);
}

#[test]
fn test_notate_line_single_span_with_padding() {
    let spans = Spans {
        pattern: "abcd\nefgh",
        line_number_width: 2,
        by_line: vec![
            vec![ast::Span { start: Position { column: 3 }, end: Position { column: 4 } }],
            vec![],
        ],
        multi_line: vec![],
    };
    let result = spans.notate_line(0);
}

#[test]
fn test_notate_line_multiple_spans_with_padding() {
    let spans = Spans {
        pattern: "abc\ndefg",
        line_number_width: 2,
        by_line: vec![
            vec![ast::Span { start: Position { column: 2 }, end: Position { column: 3 } }],
            vec![ast::Span { start: Position { column: 1 }, end: Position { column: 4 } }],
        ],
        multi_line: vec![],
    };
    let result = spans.notate_line(1);
}

#[test]
fn test_notate_line_edge_case() {
    let spans = Spans {
        pattern: "xyz\nuvw",
        line_number_width: 2,
        by_line: vec![
            vec![],
            vec![ast::Span { start: Position { column: 1 }, end: Position { column: 1 } }],
        ],
        multi_line: vec![],
    };
    let result = spans.notate_line(1);
}

#[test]
fn test_notate_line_with_multiple_lines_and_spans() {
    let spans = Spans {
        pattern: "line1\nline2\nline3",
        line_number_width: 2,
        by_line: vec![
            vec![ast::Span { start: Position { column: 3 }, end: Position { column: 4 } }],
            vec![ast::Span { start: Position { column: 2 }, end: Position { column: 7 } }],
            vec![],
        ],
        multi_line: vec![],
    };
    let result = spans.notate_line(1);
}

