// Answer 0

#[test]
fn test_column_initial_position() {
    struct DummyParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: self.pos.clone() }
        }
    }

    let position = Position { offset: 0, line: 1, column: 1 };
    let dummy_parser = DummyParser { pos: Cell::new(position) };
    let parser_i = ParserI::new(dummy_parser, "a");

    assert_eq!(parser_i.column(), 1);
}

#[test]
fn test_column_after_newline() {
    struct DummyParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: self.pos.clone() }
        }
    }

    let position = Position { offset: 5, line: 2, column: 1 };
    let dummy_parser = DummyParser { pos: Cell::new(position) };
    let parser_i = ParserI::new(dummy_parser, "a\nb");

    assert_eq!(parser_i.column(), 1);
}

#[test]
fn test_column_increment() {
    struct DummyParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: self.pos.clone() }
        }
    }

    let position = Position { offset: 2, line: 1, column: 3 };
    let dummy_parser = DummyParser { pos: Cell::new(position) };
    let parser_i = ParserI::new(dummy_parser, "abc");

    assert_eq!(parser_i.column(), 3);
}

