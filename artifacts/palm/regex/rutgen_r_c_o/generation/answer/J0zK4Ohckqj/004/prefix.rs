// Answer 0

#[test]
fn test_notate_empty_pattern_with_zero_line_number_width() {
    let pattern = "";
    let line_number_width = 0;
    let by_line = vec![vec![]];
    let multi_line = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.notate();
}

#[test]
fn test_notate_single_line_pattern_with_zero_line_number_width() {
    let pattern = "abc";
    let line_number_width = 0;
    let by_line = vec![vec![]];
    let multi_line = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.notate();
}

#[test]
fn test_notate_multi_line_pattern_with_zero_line_number_width() {
    let pattern = "abc\ndef";
    let line_number_width = 0;
    let by_line = vec![vec![], vec![]];
    let multi_line = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.notate();
}

#[test]
fn test_notate_single_line_with_span() {
    let pattern = "abc";
    let line_number_width = 2;
    let by_line = vec![vec![Span { start: Position::new(0, 1), end: Position::new(0, 2) }]];
    let multi_line = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.notate();
}

#[test]
fn test_notate_multi_line_with_spans() {
    let pattern = "abc\ndef";
    let line_number_width = 3;
    let by_line = vec![
        vec![Span { start: Position::new(0, 1), end: Position::new(0, 2) }],
        vec![Span { start: Position::new(1, 1), end: Position::new(1, 2) }],
    ];
    let multi_line = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.notate();
}

#[test]
fn test_notate_no_spans() {
    let pattern = "abc";
    let line_number_width = 2;
    let by_line = vec![vec![]];
    let multi_line = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.notate();
}

