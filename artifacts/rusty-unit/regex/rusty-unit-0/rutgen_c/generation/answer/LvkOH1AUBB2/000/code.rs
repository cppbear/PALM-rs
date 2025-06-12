// Answer 0

#[test]
fn test_notate_line_empty_spans() {
    let spans = Spans {
        pattern: "abc",
        line_number_width: 2,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    assert_eq!(spans.notate_line(0), None);
}

#[test]
fn test_notate_line_single_span() {
    let ast_span = ast::Span {
        start: Position { column: 2 },
        end: Position { column: 4 },
    };
    let spans = Spans {
        pattern: "abc",
        line_number_width: 2,
        by_line: vec![vec![ast_span]],
        multi_line: vec![],
    };
    assert_eq!(spans.notate_line(0), Some("  ^^^".to_string()));
}

#[test]
fn test_notate_line_multiple_spans() {
    let ast_span1 = ast::Span {
        start: Position { column: 2 },
        end: Position { column: 4 },
    };
    let ast_span2 = ast::Span {
        start: Position { column: 6 },
        end: Position { column: 8 },
    };
    let spans = Spans {
        pattern: "abc def",
        line_number_width: 2,
        by_line: vec![vec![ast_span1, ast_span2]],
        multi_line: vec![],
    };
    assert_eq!(spans.notate_line(0), Some("  ^^^   ^^^".to_string()));
}

#[test]
fn test_notate_line_with_line_number_padding() {
    let ast_span = ast::Span {
        start: Position { column: 5 },
        end: Position { column: 7 },
    };
    let spans = Spans {
        pattern: "abcde fgh",
        line_number_width: 3,
        by_line: vec![vec![ast_span]],
        multi_line: vec![],
    };
    assert_eq!(spans.notate_line(0), Some("    ^^^".to_string()));
}

