// Answer 0

#[test]
fn test_parse_escape_octal() {
    struct MockParser {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl MockParser {
        fn new(pattern: &str, pos_offset: usize, pos_line: usize, pos_column: usize) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position {
                    offset: pos_offset,
                    line: pos_line,
                    column: pos_column,
                },
                octal: true,
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap()
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn parse_octal(&self) -> ast::Literal {
            ast::Literal {
                span: Span::new(self.pos, self.pos), // Simplified for the test
                kind: ast::LiteralKind::Octal,
                c: '8', // Simplified output
            }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos) // Simplified for the test
        }
        
        // Rest of the methods are omitted for brevity
    }

    let mut parser = MockParser::new("\\123", 0, 1, 1);
    let result = parser.parse_escape();
    
    assert!(result.is_ok());
    if let Ok(primitive) = result {
        if let Primitive::Literal(lit) = primitive {
            assert_eq!(lit.kind, ast::LiteralKind::Octal);
        } else {
            panic!("Expected Literal.");
        }
    }
}

#[test]
fn test_parse_escape_invalid_backreference() {
    struct MockParser {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl MockParser {
        fn new(pattern: &str, pos_offset: usize, pos_line: usize, pos_column: usize) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position {
                    offset: pos_offset,
                    line: pos_line,
                    column: pos_column,
                },
                octal: false, // Disabling octal for this test
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap()
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }
        
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos) // Simplified for the test
        }
    }

    let mut parser = MockParser::new("\\8", 0, 1, 1);
    let result = parser.parse_escape();
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::UnsupportedBackreference);
    }
}

#[test]
fn test_parse_escape_special_character() {
    struct MockParser {
        pattern: String,
        pos: Position,
    }

    impl MockParser {
        fn new(pattern: &str, pos_offset: usize, pos_line: usize, pos_column: usize) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position {
                    offset: pos_offset,
                    line: pos_line,
                    column: pos_column,
                },
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap()
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }
        
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos) // Simplified for the test
        }
    }

    let mut parser = MockParser::new("\\n", 0, 1, 1);
    let result = parser.parse_escape();

    assert!(result.is_ok());
    if let Ok(primitive) = result {
        if let Primitive::Literal(lit) = primitive {
            assert_eq!(lit.kind, ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed));
        } else {
            panic!("Expected Literal.");
        }
    }
}

