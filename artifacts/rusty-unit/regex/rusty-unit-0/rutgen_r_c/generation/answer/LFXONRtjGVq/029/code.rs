// Answer 0

#[test]
fn test_parse_counted_repetition_empty_concat() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let start_span = Span::new(position, position);
    
    struct ParserMock {
        char: char,
        pos: Cell<Position>,
        ast: Ast,
    }

    impl ParserMock {
        fn new() -> Self {
            Self { char: '{', pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), ast: Ast::Empty(start_span) }
        }
        
        fn char(&self) -> char {
            self.char
        }
        
        fn pos(&self) -> Position {
            self.pos.get()
        }
        
        fn bump_and_bump_space(&self) -> bool {
            false
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::RepetitionCountUnclosed, pattern: "".to_string(), span: start_span }
        }
        
        fn is_eof(&self) -> bool {
            false
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            Err(self.error(start_span, ast::ErrorKind::RepetitionCountUnclosed).into())
        }
    }
    
    let mut concat = ast::Concat { span: start_span, asts: vec![Ast::Empty(start_span)] };
    let parser = ParserMock::new();
    
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_char_not_open_brace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let start_span = Span::new(position, position);
    
    struct ParserMock {
        char: char,
        pos: Cell<Position>,
        ast: Ast,
    }

    impl ParserMock {
        fn new(char: char) -> Self {
            Self { char, pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), ast: Ast::Empty(start_span) }
        }
        
        fn char(&self) -> char {
            self.char
        }
        
        fn pos(&self) -> Position {
            self.pos.get()
        }
        
        fn bump_and_bump_space(&self) -> bool {
            true
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::RepetitionCountUnclosed, pattern: "".to_string(), span: start_span }
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            Err(self.error(start_span, ast::ErrorKind::RepetitionCountUnclosed).into())
        }
        
        fn is_eof(&self) -> bool {
            false
        }
    }
    
    let mut concat = ast::Concat { span: start_span, asts: vec![Ast::Empty(start_span)] };
    let parser = ParserMock::new('a'); // Character is not '{'
    
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_bump_and_bump_space_false() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let start_span = Span::new(position, position);
    
    struct ParserMock {
        char: char,
        pos: Cell<Position>,
        ast: Ast,
    }

    impl ParserMock {
        fn new() -> Self {
            Self { char: '{', pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), ast: Ast::Empty(start_span) }
        }
        
        fn char(&self) -> char {
            self.char
        }
        
        fn pos(&self) -> Position {
            self.pos.get()
        }
        
        fn bump_and_bump_space(&self) -> bool {
            false
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::RepetitionCountUnclosed, pattern: "".to_string(), span: start_span }
        }
        
        fn is_eof(&self) -> bool {
            false
        }

        fn parse_decimal(&self) -> Result<u32> {
            Ok(1)
        }
    }
    
    let mut concat = ast::Concat { span: start_span, asts: vec![Ast::Empty(start_span)] };
    let parser = ParserMock::new();
    
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

