// Answer 0

#[test]
fn test_span_char_newline() {
    struct DummyParser {
        offset: usize,
        line: usize,
        column: usize,
    }
    
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            // Create dummy parser to fulfill Borrow trait
            &Parser { 
                pos: Cell::new(Position { offset: self.offset, line: self.line, column: self.column }), 
                ..Parser::default() 
            }
        }
    }

    let parser = DummyParser { offset: 0, line: 1, column: 1 };
    let parser_i = ParserI::new(parser, "abc\n");
    
    let span = parser_i.span_char();
    assert_eq!(span.start.offset, 0);
    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 1);
    assert_eq!(span.end.offset, 1);
    assert_eq!(span.end.line, 2);
    assert_eq!(span.end.column, 1);
}

#[test]
#[should_panic]
fn test_span_char_panic_offset() {
    struct DummyParser {
        offset: usize,
        line: usize,
        column: usize,
    }
    
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            // Create dummy parser to fulfill Borrow trait
            &Parser { 
                pos: Cell::new(Position { offset: usize::MAX, line: self.line, column: self.column }), 
                ..Parser::default() 
            }
        }
    }

    let parser = DummyParser { offset: usize::MAX, line: 1, column: 1 };
    let parser_i = ParserI::new(parser, "a");
    
    // This should cause a panic due to overflow in checked_add
    let _span = parser_i.span_char();
}

#[test]
#[should_panic]
fn test_span_char_panic_column() {
    struct DummyParser {
        offset: usize,
        line: usize,
        column: usize,
    }
    
    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            // Create dummy parser to fulfill Borrow trait
            &Parser { 
                pos: Cell::new(Position { offset: 0, line: self.line, column: usize::MAX }), 
                ..Parser::default() 
            }
        }
    }

    let parser = DummyParser { offset: 0, line: 1, column: usize::MAX };
    let parser_i = ParserI::new(parser, "a\n");
    
    // This should cause a panic due to overflow in checked_add
    let _span = parser_i.span_char();
}

