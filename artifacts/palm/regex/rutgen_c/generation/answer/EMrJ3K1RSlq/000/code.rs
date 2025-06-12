// Answer 0

#[test]
fn test_add_one_line_span() {
    struct DummyAstSpan {
        start: Position,
        end: Position,
    }
    
    impl DummyAstSpan {
        fn new(start: Position, end: Position) -> Self {
            DummyAstSpan { start, end }
        }
    }

    let start_pos = Position { line: 1, column: 0 };
    let end_pos = Position { line: 1, column: 5 };
    let span = DummyAstSpan::new(start_pos, end_pos);
    
    let mut spans = Spans {
        pattern: "test pattern",
        line_number_width: 2,
        by_line: vec![vec![], vec![]],
        multi_line: vec![],
    };
    
    spans.add(span);
    
    assert_eq!(spans.by_line[0].len(), 1);
    assert!(spans.by_line[0][0].start == start_pos);
    assert!(spans.by_line[0][0].end == end_pos);
}

#[test]
fn test_add_multi_line_span() {
    struct DummyAstSpan {
        start: Position,
        end: Position,
    }
    
    impl DummyAstSpan {
        fn new(start: Position, end: Position) -> Self {
            DummyAstSpan { start, end }
        }
    }

    let start_pos = Position { line: 1, column: 0 };
    let end_pos = Position { line: 2, column: 5 };
    let span = DummyAstSpan::new(start_pos, end_pos);
    
    let mut spans = Spans {
        pattern: "test pattern",
        line_number_width: 2,
        by_line: vec![vec![], vec![]],
        multi_line: vec![],
    };
    
    spans.add(span);
    
    assert_eq!(spans.multi_line.len(), 1);
    assert!(spans.multi_line[0].start == start_pos);
    assert!(spans.multi_line[0].end == end_pos);
}

