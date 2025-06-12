// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct MockParser {
        pos: Position,
        pattern: String,
        eof: bool,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            '>' // trigger valid end
        }

        fn bump(&mut self) -> bool {
            false // simulate no-bump scenario
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos) // span at current position
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::__Nonexhaustive,
                pattern: self.pattern.clone(),
                span: self.span(),
            }
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<()> {
            Ok(()) // simulate successful addition
        }
    }

    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let mock_parser = MockParser {
        pos: start,
        pattern: "<valid_name>".to_string(),
        eof: false,
    };

    let parser_instance = ParserI {
        parser: &mock_parser,
        pattern: &mock_parser.pattern,
    };

    let result = parser_instance.parse_capture_name(1);
    
    assert!(result.is_ok());
    let capname = result.unwrap();
    assert_eq!(capname.name, "valid_name");
    assert_eq!(capname.index, 1);
}

#[test]
fn test_parse_capture_name_empty_name() {
    struct MockParser {
        pos: Position,
        pattern: String,
        eof: bool,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            '>' // trigger valid end
        }

        fn bump(&mut self) -> bool {
            false // simulate no-bump scenario
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos) // span at current position
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span: self.span(),
            }
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<()> {
            Ok(()) // simulate successful addition
        }
    }

    let start = Position { offset: 0, line: 1, column: 1 };
    let mock_parser = MockParser {
        pos: start,
        pattern: "<>".to_string(), // empty capture name
        eof: false,
    };

    let parser_instance = ParserI {
        parser: &mock_parser,
        pattern: &mock_parser.pattern,
    };

    let result = parser_instance.parse_capture_name(1);
    
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_name_invalid_character() {
    struct MockParser {
        pos: Position,
        pattern: String,
        eof: bool,
        should_bump_fail: bool,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            '3'  // invalid character for starting name
        }

        fn bump(&mut self) -> bool {
            if self.should_bump_fail {
                false // simulate no-bump
            } else {
                true // simulate bump success
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos) // span at current position
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span: self.span(),
            }
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<()> {
            Ok(()) // simulate successful addition
        }
    }

    let start = Position { offset: 0, line: 1, column: 1 };
    let mock_parser = MockParser {
        pos: start,
        pattern: "<3name>".to_string(), // invalid capture name starts with digit
        eof: false,
        should_bump_fail: false,
    };

    let parser_instance = ParserI {
        parser: &mock_parser,
        pattern: &mock_parser.pattern,
    };

    let result = parser_instance.parse_capture_name(1);

    assert!(result.is_err());
}

