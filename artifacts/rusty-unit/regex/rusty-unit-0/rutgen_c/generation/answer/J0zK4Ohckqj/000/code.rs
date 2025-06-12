// Answer 0

#[test]
fn test_notate_single_line_no_spans() {
    struct DummySpan {
        start: Position,
        end: Position,
    }
    
    let pattern = "abc";
    let by_line: Vec<Vec<DummySpan>> = vec![vec![]];
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate();
    assert_eq!(result, "    abc\n");
}

#[test]
fn test_notate_single_line_with_spans() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    let pattern = "abc";
    let by_line: Vec<Vec<DummySpan>> = vec![vec![DummySpan { start: Position { column: 1 }, end: Position { column: 2 } }]];
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate();
    assert_eq!(result, "    abc\n    ^\n");
}

#[test]
fn test_notate_single_line_with_multiple_spans() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    let pattern = "abc";
    let by_line: Vec<Vec<DummySpan>> = vec![
        vec![
            DummySpan { start: Position { column: 1 }, end: Position { column: 2 } },
            DummySpan { start: Position { column: 3 }, end: Position { column: 3 } },
        ]
    ];
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate();
    assert_eq!(result, "    abc\n    ^ \n");
}

#[test]
fn test_notate_multiple_lines_with_no_spans() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    let pattern = "abc\ndef";
    let by_line: Vec<Vec<DummySpan>> = vec![vec![], vec![]];
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate();
    assert_eq!(result, "    abc\n    def\n");
}

#[test]
fn test_notate_multiple_lines_with_spans() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    let pattern = "abc\ndef";
    let by_line: Vec<Vec<DummySpan>> = vec![
        vec![DummySpan { start: Position { column: 1 }, end: Position { column: 2 } }],
        vec![DummySpan { start: Position { column: 1 }, end: Position { column: 3 } }]
    ];
    let multi_line: Vec<DummySpan> = vec![];

    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate();
    assert_eq!(result, "    abc\n    ^\n    def\n    ^^^\n");
}

