// Answer 0

#[test]
fn test_add_one_line_span_with_zero_line_number_width() {
    let pattern = "a";
    let line_number_width = 0;
    let mut by_line = vec![vec![]; 1];
    let multi_line = vec![];
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    let start = Position { line: 1, column: 1 }; // Assuming the Position struct with line and column
    let end = Position { line: 1, column: 2 };
    let span = Span::new(start, end);
    spans.add(span);
}

#[test]
fn test_add_one_line_span_with_non_zero_line_number_width() {
    let pattern = "abc\ndef\nghi";
    let line_number_width = 5;
    let mut by_line = vec![vec![], vec![], vec![]];
    let multi_line = vec![];
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    let start = Position { line: 1, column: 1 };
    let end = Position { line: 1, column: 2 };
    let span = Span::new(start, end);
    spans.add(span);
}

#[test]
fn test_add_multiple_one_line_spans() {
    let pattern = "r\ns\ntest";
    let line_number_width = 2;
    let mut by_line = vec![vec![], vec![], vec![]];
    let multi_line = vec![];
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    let span1 = Span::new(Position { line: 1, column: 1 }, Position { line: 1, column: 2 });
    let span2 = Span::new(Position { line: 1, column: 3 }, Position { line: 1, column: 4 });
    spans.add(span1);
    spans.add(span2);
}

#[test]
fn test_add_one_line_span_to_empty_line() {
    let pattern = "abc\n\nxyz";
    let line_number_width = 3;
    let mut by_line = vec![vec![ ]; vec![ ]; vec![]]; 
    let multi_line = vec![];
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    let start = Position { line: 2, column: 1 };
    let end = Position { line: 2, column: 2 };
    let span = Span::new(start, end);
    spans.add(span);
}

#[test]
fn test_add_one_line_span_to_full_line() {
    let pattern = "full line";
    let line_number_width = 1;
    let mut by_line = vec![vec![]];
    let multi_line = vec![];
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    let start = Position { line: 1, column: 1 };
    let end = Position { line: 1, column: 10 };
    let span = Span::new(start, end);
    spans.add(span);
}

