// Answer 0

#[test]
fn test_notate_line_single_span() {
    let span = ast::Span { start: Position { column: 3 }, end: Position { column: 5 } };
    let spans = vec![vec![span]];
    let by_line = spans.to_vec();
    let line_number_width = 2;
    let spans_struct = Spans { pattern: "test", line_number_width, by_line, multi_line: vec![] };
    let _ = spans_struct.notate_line(0);
}

#[test]
fn test_notate_line_multiple_spans() {
    let span1 = ast::Span { start: Position { column: 2 }, end: Position { column: 4 } };
    let span2 = ast::Span { start: Position { column: 8 }, end: Position { column: 10 } };
    let spans = vec![vec![span1, span2]];
    let line_number_width = 3;
    let spans_struct = Spans { pattern: "multiple spans", line_number_width, by_line: spans, multi_line: vec![] };
    let _ = spans_struct.notate_line(0);
}

#[test]
fn test_notate_line_no_initial_padding() {
    let span = ast::Span { start: Position { column: 1 }, end: Position { column: 4 } };
    let spans = vec![vec![span]];
    let line_number_width = 0; // No padding for line numbers
    let spans_struct = Spans { pattern: "no padding", line_number_width, by_line: spans, multi_line: vec![] };
    let _ = spans_struct.notate_line(0);
}

#[test]
fn test_notate_line_with_padding_for_large_column() {
    let span = ast::Span { start: Position { column: 10 }, end: Position { column: 12 } };
    let spans = vec![vec![span]];
    let line_number_width = 5; // Sufficient width for padding
    let spans_struct = Spans { pattern: "large column number", line_number_width, by_line: spans, multi_line: vec![] };
    let _ = spans_struct.notate_line(0);
}

#[test]
fn test_notate_line_empty_span_case() {
    let spans: Vec<ast::Span> = vec![];
    let by_line = vec![spans]; // No spans in this line
    let line_number_width = 2;
    let spans_struct = Spans { pattern: "no spans", line_number_width, by_line: by_line, multi_line: vec![] };
    let result = spans_struct.notate_line(0);
    let _ = result; // Expecting None, but this handles the 'empty case'
}

