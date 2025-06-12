// Answer 0

#[test]
fn test_parse_uncounted_repetition_no_previous_ast() {
    struct TestParser {
        pos: Position,
        asts: Vec<Ast>,
    }

    impl TestParser {
        fn char(&self) -> char {
            '?'
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump(&mut self) -> bool {
            false
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error { kind, pattern: String::new(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let parser = TestParser { 
        pos: Position { offset: 0, line: 1, column: 1 }, 
        asts: vec![] 
    };

    let concat = ast::Concat { 
        span: Span::new(parser.pos, parser.pos), 
        asts: parser.asts 
    };

    let result = parser.parse_uncounted_repetition(concat, RepetitionKind::ZeroOrMore);
    
    match result {
        Err(err) => {
            assert_eq!(err.kind, ErrorKind::RepetitionMissing);
        }
        _ => panic!("Expected an error, but got a different result."),
    }
}

#[test]
fn test_parse_uncounted_repetition_invalid_char() {
    struct TestParser {
        pos: Position,
        asts: Vec<Ast>,
    }

    impl TestParser {
        fn char(&self) -> char {
            'a' // setting a character that is not a repetition operator
        }
        
        fn pos(&self) -> Position {
            self.pos
        }

        fn bump(&mut self) -> bool {
            false
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error { kind, pattern: String::new(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let parser = TestParser { 
        pos: Position { offset: 0, line: 1, column: 1 }, 
        asts: vec![Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))] 
    };

    let concat = ast::Concat { 
        span: Span::new(parser.pos, parser.pos), 
        asts: parser.asts 
    };

    let result = parser.parse_uncounted_repetition(concat, RepetitionKind::ZeroOrMore);
    
    match result {
        Err(err) => {
            assert_eq!(err.kind, ErrorKind::RepetitionMissing);
        }
        _ => panic!("Expected an error, but got a different result."),
    }
}

