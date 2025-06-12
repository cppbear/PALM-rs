// Answer 0

#[test]
fn test_notate_line_empty_span_first_line() {
    let by_line: Vec<Vec<ast::Span>> = vec![Vec::new()];
    let spans = Spans {
        pattern: "abc",
        line_number_width: 2,
        by_line,
        multi_line: Vec::new(),
    };
    let result = spans.notate_line(0);
}

#[test]
fn test_notate_line_empty_span_middle_line() {
    let by_line: Vec<Vec<ast::Span>> = vec![vec![ast::Span { start: Position { column: 1 }, end: Position { column: 1 } }],
                                             Vec::new(),
                                             vec![ast::Span { start: Position { column: 2 }, end: Position { column: 3 } }]];
    let spans = Spans {
        pattern: "abc\n\nabc",
        line_number_width: 2,
        by_line,
        multi_line: Vec::new(),
    };
    let result = spans.notate_line(1);
}

#[test]
fn test_notate_line_empty_span_last_line() {
    let by_line: Vec<Vec<ast::Span>> = vec![vec![ast::Span { start: Position { column: 1 }, end: Position { column: 1 } }],
                                             vec![ast::Span { start: Position { column: 1 }, end: Position { column: 1 } }],
                                             Vec::new()];
    let spans = Spans {
        pattern: "a\nb\n",
        line_number_width: 2,
        by_line,
        multi_line: Vec::new(),
    };
    let result = spans.notate_line(2);
}

#[test]
fn test_notate_line_index_out_of_bounds() {
    let by_line: Vec<Vec<ast::Span>> = vec![Vec::new()];
    let spans = Spans {
        pattern: "abc",
        line_number_width: 2,
        by_line,
        multi_line: Vec::new(),
    };
    let result = spans.notate_line(1);
}

