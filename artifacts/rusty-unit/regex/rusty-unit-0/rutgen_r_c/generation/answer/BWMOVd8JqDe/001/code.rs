// Answer 0

#[test]
fn test_span() {
    struct MockParser {
        pos: Cell<Position>,
    }

    impl MockParser {
        fn new(offset: usize, line: usize, column: usize) -> Self {
            Self {
                pos: Cell::new(Position {
                    offset,
                    line,
                    column,
                }),
            }
        }
    }

    let parser_instance = MockParser::new(0, 1, 1);
    let parser_i = ParserI::new(&parser_instance, "test_pattern");

    let result_span = parser_i.span();
    assert_eq!(result_span.start.offset, 0);
    assert_eq!(result_span.start.line, 1);
    assert_eq!(result_span.start.column, 1);
    assert_eq!(result_span.end.offset, 0);
    assert_eq!(result_span.end.line, 1);
    assert_eq!(result_span.end.column, 1);
}

#[test]
fn test_span_with_non_zero_position() {
    struct MockParser {
        pos: Cell<Position>,
    }

    impl MockParser {
        fn new(offset: usize, line: usize, column: usize) -> Self {
            Self {
                pos: Cell::new(Position {
                    offset,
                    line,
                    column,
                }),
            }
        }
    }

    let parser_instance = MockParser::new(10, 3, 15);
    let parser_i = ParserI::new(&parser_instance, "test_pattern");

    let result_span = parser_i.span();
    assert_eq!(result_span.start.offset, 10);
    assert_eq!(result_span.start.line, 3);
    assert_eq!(result_span.start.column, 15);
    assert_eq!(result_span.end.offset, 10);
    assert_eq!(result_span.end.line, 3);
    assert_eq!(result_span.end.column, 15);
}

#[test]
fn test_span_with_boundary_position() {
    struct MockParser {
        pos: Cell<Position>,
    }

    impl MockParser {
        fn new(offset: usize, line: usize, column: usize) -> Self {
            Self {
                pos: Cell::new(Position {
                    offset,
                    line,
                    column,
                }),
            }
        }
    }

    let parser_instance = MockParser::new(usize::MAX, 1, 1);
    let parser_i = ParserI::new(&parser_instance, "test_pattern");

    let result_span = parser_i.span();
    assert_eq!(result_span.start.offset, usize::MAX);
    assert_eq!(result_span.start.line, 1);
    assert_eq!(result_span.start.column, 1);
    assert_eq!(result_span.end.offset, usize::MAX);
    assert_eq!(result_span.end.line, 1);
    assert_eq!(result_span.end.column, 1);
}

