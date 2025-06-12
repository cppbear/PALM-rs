// Answer 0

#[test]
fn test_column_initial_position() {
    struct MockParser {
        pos: Position,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.pos) }
        }
    }
    
    let parser = MockParser { pos: Position { offset: 0, line: 1, column: 1 } };
    let parser_i = ParserI::new(parser, "abc");
    
    assert_eq!(parser_i.column(), 1);
}

#[test]
fn test_column_mid_line_position() {
    struct MockParser {
        pos: Position,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.pos) }
        }
    }
    
    let parser = MockParser { pos: Position { offset: 5, line: 1, column: 6 } };
    let parser_i = ParserI::new(parser, "abcdef");
    
    assert_eq!(parser_i.column(), 6);
}

#[test]
fn test_column_new_line_position() {
    struct MockParser {
        pos: Position,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.pos) }
        }
    }
    
    let parser = MockParser { pos: Position { offset: 0, line: 2, column: 1 } };
    let parser_i = ParserI::new(parser, "\nabc");
    
    assert_eq!(parser_i.column(), 1);
}

#[test]
fn test_column_after_line_feed() {
    struct MockParser {
        pos: Position,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser { pos: Cell::new(self.pos) }
        }
    }
    
    let parser = MockParser { pos: Position { offset: 11, line: 2, column: 5 } };
    let parser_i = ParserI::new(parser, "\nabc\ndef");
    
    assert_eq!(parser_i.column(), 5);
}

