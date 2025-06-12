// Answer 0

#[test]
fn test_parse_counted_repetition_char_not_opening_brace() {
    struct DummyParser<'s> {
        char: char,
        pos: Position,
    }
    impl<'s> DummyParser<'s> {
        fn char(&self) -> char {
            self.char
        }
        fn pos(&self) -> Position {
            self.pos
        }
        fn is_eof(&self) -> bool {
            false
        }
        fn bump_and_bump_space(&self) -> bool {
            true
        }
    }

    let parser = DummyParser {
        char: 'a',
        pos: Position { offset: 0, line: 1, column: 1 },
    };
    
    let concat = ast::Concat {
        span: Span { start: parser.pos(), end: parser.pos() },
        asts: vec![ast::Ast::Empty(Span::new(parser.pos(), parser.pos()))],
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_pop_none() {
    struct DummyParser<'s> {
        char: char,
        pos: Position,
    }
    impl<'s> DummyParser<'s> {
        fn char(&self) -> char {
            '{'
        }
        fn pos(&self) -> Position {
            self.pos
        }
        fn is_eof(&self) -> bool {
            false
        }
        fn bump_and_bump_space(&self) -> bool {
            true
        }
        fn concat_with_none(&self) -> ast::Concat {
            ast::Concat {
                span: Span { start: self.pos(), end: self.pos() },
                asts: vec![],
            }
        }
    }

    let parser = DummyParser {
        char: '{',
        pos: Position { offset: 0, line: 1, column: 1 },
    };
    
    let concat = parser.concat_with_none();

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_bump_and_bump_space_false() {
    struct DummyParser<'s> {
        char: char,
        pos: Position,
        bump_success: bool,
    }
    impl<'s> DummyParser<'s> {
        fn char(&self) -> char {
            '{'
        }
        fn pos(&self) -> Position {
            self.pos
        }
        fn is_eof(&self) -> bool {
            false
        }
        fn bump_and_bump_space(&self) -> bool {
            self.bump_success
        }
        fn parse_decimal(&self) -> Result<u32> {
            Ok(3)
        }
    }

    let parser = DummyParser {
        char: '{',
        pos: Position { offset: 0, line: 1, column: 1 },
        bump_success: false,
    };

    let concat = ast::Concat {
        span: Span { start: parser.pos(), end: parser.pos() },
        asts: vec![ast::Ast::Empty(Span::new(parser.pos(), parser.pos()))],
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

