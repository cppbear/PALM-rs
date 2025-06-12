// Answer 0

#[test]
fn test_add_multiline_span_valid() {
    let mut spans = Spans {
        pattern: "some\nregex\npattern",
        line_number_width: 2,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let start_position = Position { line: 1, column: 0 }; 
    let end_position = Position { line: 3, column: 0 }; 
    let span = Span::new(start_position, end_position);
    spans.add(span);
}

#[test]
fn test_add_span_with_non_one_line_coordinates() {
    let mut spans = Spans {
        pattern: "line 1\nline 2\nline 3",
        line_number_width: 2,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let start_position = Position { line: 2, column: 0 }; 
    let end_position = Position { line: 4, column: 0 }; 
    let span = Span::new(start_position, end_position);
    spans.add(span);
}

#[test]
fn test_add_span_with_edge_case_lines() {
    let mut spans = Spans {
        pattern: "first line\nsecond line\nthird line",
        line_number_width: 0,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let start_position = Position { line: 1, column: 0 }; 
    let end_position = Position { line: 2, column: 5 }; 
    let span = Span::new(start_position, end_position);
    spans.add(span);
}

#[test]
fn test_add_span_with_multiple_lines() {
    let mut spans = Spans {
        pattern: "abc\ndef\nghi\njkl",
        line_number_width: 3,
        by_line: vec![vec![], vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let start_position = Position { line: 2, column: 0 }; 
    let end_position = Position { line: 4, column: 0 }; 
    let span = Span::new(start_position, end_position);
    spans.add(span);
}

#[test]
fn test_add_span_with_non_contiguous_lines() {
    let mut spans = Spans {
        pattern: "line1\nline2\nline3",
        line_number_width: 1,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let start_position = Position { line: 3, column: 5 }; 
    let end_position = Position { line: 4, column: 1 }; 
    let span = Span::new(start_position, end_position);
    spans.add(span);
}

