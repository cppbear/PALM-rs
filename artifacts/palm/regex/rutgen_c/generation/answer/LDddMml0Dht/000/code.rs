// Answer 0

#[test]
fn test_bump_if_prefix_matches() {
    struct DummyParser {
        pos: Position,
        pattern: String,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.pos) }
        }
    }

    let pattern = "hello world";
    let parser = DummyParser { pos: Position { offset: 0, line: 1, column: 1 }, pattern: pattern.to_string() };
    let parser_i = ParserI::new(parser, &parser.pattern);
    
    assert!(parser_i.bump_if("hello"));
    assert_eq!(parser_i.offset(), 5);
}

#[test]
fn test_bump_if_prefix_does_not_match() {
    struct DummyParser {
        pos: Position,
        pattern: String,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.pos) }
        }
    }

    let pattern = "hello world";
    let parser = DummyParser { pos: Position { offset: 0, line: 1, column: 1 }, pattern: pattern.to_string() };
    let parser_i = ParserI::new(parser, &parser.pattern);
    
    assert!(!parser_i.bump_if("world"));
    assert_eq!(parser_i.offset(), 0);
}

#[test]
fn test_bump_if_empty_prefix() {
    struct DummyParser {
        pos: Position,
        pattern: String,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.pos) }
        }
    }

    let pattern = "hello world";
    let parser = DummyParser { pos: Position { offset: 0, line: 1, column: 1 }, pattern: pattern.to_string() };
    let parser_i = ParserI::new(parser, &parser.pattern);
    
    assert!(parser_i.bump_if(""));
    assert_eq!(parser_i.offset(), 0);
}

