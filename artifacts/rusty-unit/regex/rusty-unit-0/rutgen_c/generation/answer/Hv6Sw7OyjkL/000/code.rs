// Answer 0

#[test]
fn test_peek_empty_pattern() {
    struct TestParser {
        pos: Cell<Position>,
        // other fields can be initialized as needed
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // dummy implementation
            unimplemented!()
        }
    }

    let test_parser = TestParser {
        pos: Cell::new(Position { offset: 0 }), // assuming Position has an offset field
    };

    let parser_i = ParserI::new(test_parser, "");
    assert_eq!(parser_i.peek(), None);
}

#[test]
fn test_peek_single_character() {
    struct TestParser {
        pos: Cell<Position>,
        // other fields can be initialized as needed
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // dummy implementation
            unimplemented!()
        }
    }

    let test_parser = TestParser {
        pos: Cell::new(Position { offset: 0 }),
    };

    let parser_i = ParserI::new(test_parser, "a");
    assert_eq!(parser_i.peek(), Some('a'));
}

#[test]
fn test_peek_multiple_characters() {
    struct TestParser {
        pos: Cell<Position>,
        // other fields can be initialized as needed
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // dummy implementation
            unimplemented!()
        }
    }

    let test_parser = TestParser {
        pos: Cell::new(Position { offset: 0 }),
    };

    let parser_i = ParserI::new(test_parser, "abc");
    assert_eq!(parser_i.peek(), Some('a'));    
}

#[test]
fn test_peek_at_eof() {
    struct TestParser {
        pos: Cell<Position>,
        // other fields can be initialized as needed
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // dummy implementation
            unimplemented!()
        }
    }

    let test_parser = TestParser {
        pos: Cell::new(Position { offset: 3 }), // offset at end of "abc"
    };

    let parser_i = ParserI::new(test_parser, "abc");
    assert_eq!(parser_i.peek(), None);
}

