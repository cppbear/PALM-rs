// Answer 0

#[test]
fn test_parse_counted_repetition_char_not_opening_brace() {
    struct MockParser {
        pos: Position,
        parser_state: String,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                parser_state: "a".to_string(), // Not starting with '{'
            }
        }

        fn char(&self) -> char {
            self.parser_state.chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: "test".to_string(), span: Span::new(self.pos, self.pos) }
        }

        fn is_eof(&self) -> bool {
            self.parser_state.is_empty()
        }
    }

    let mock_parser = MockParser::new();
    let concat = ast::Concat { span: Span::new(mock_parser.pos, mock_parser.pos), asts: Vec::new() };
    let result = mock_parser.parse_counted_repetition(concat);
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::RepetitionMissing);
    }
}

#[test]
fn test_parse_counted_repetition_empty_concat() {
    struct MockParser {
        pos: Position,
        parser_state: String,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                parser_state: "{".to_string(), // Starts with '{'
            }
        }
        
        fn char(&self) -> char {
            self.parser_state.chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: "test".to_string(), span: Span::new(self.pos, self.pos) }
        }

        fn is_eof(&self) -> bool {
            self.parser_state.is_empty()
        }
    }

    let mock_parser = MockParser::new();
    let concat = ast::Concat { span: Span::new(mock_parser.pos, mock_parser.pos), asts: Vec::new() }; // Empty asts
    let result = mock_parser.parse_counted_repetition(concat);
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::RepetitionMissing);
    }
}

