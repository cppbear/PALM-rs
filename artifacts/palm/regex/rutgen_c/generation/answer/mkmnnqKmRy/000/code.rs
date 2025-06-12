// Answer 0

#[test]
fn test_offset() {
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

    let dummy_parser = DummyParser::new(5, 1, 6);
    let parser_i = ParserI::new(&dummy_parser, "test_pattern");

    assert_eq!(parser_i.offset(), 5);
}

#[test]
fn test_offset_zero() {
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

    let dummy_parser = DummyParser::new(0, 1, 1);
    let parser_i = ParserI::new(&dummy_parser, "test_pattern");

    assert_eq!(parser_i.offset(), 0);
}

