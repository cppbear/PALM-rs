// Answer 0

#[test]
fn test_span() {
    struct DummyParser {
        pos: Cell<Position>,
    }

    impl DummyParser {
        fn new(offset: usize, line: usize, column: usize) -> Self {
            DummyParser {
                pos: Cell::new(Position { offset, line, column }),
            }
        }
    }

    let parser = DummyParser::new(5, 1, 6);
    let parser_i = ParserI::new(&parser, "abc");

    let span = parser_i.span();
    assert_eq!(span.start, parser.pos.get());
    assert_eq!(span.end, parser.pos.get());
}

#[test]
fn test_span_for_empty_position() {
    struct DummyParser {
        pos: Cell<Position>,
    }

    impl DummyParser {
        fn new(offset: usize, line: usize, column: usize) -> Self {
            DummyParser {
                pos: Cell::new(Position { offset, line, column }),
            }
        }
    }

    let parser = DummyParser::new(0, 1, 1);
    let parser_i = ParserI::new(&parser, "");

    let span = parser_i.span();
    assert_eq!(span.start, parser.pos.get());
    assert_eq!(span.end, parser.pos.get());
}

