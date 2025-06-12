// Answer 0

#[test]
fn test_parse_set_class_range_valid_literal() {
    struct TestParser {
        input: String,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump_space(&mut self) {
            self.pos.offset += 1;
            self.column += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn peek_space(&self) -> Option<char> {
            if self.pos.offset + 1 < self.input.len() {
                Some(self.input.chars().nth(self.pos.offset + 1).unwrap())
            } else {
                None
            }
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            Ok(Primitive::Literal(Literal {
                span: Span::new(self.pos, self.pos),
                kind: ast::LiteralKind::Verbatim,
                c: self.char(),
            }))
        }

        fn error(&self, span: &Span, kind: ast::ErrorKind) -> Error {
            Error { kind: kind, pattern: self.input.clone(), span: span.clone() }
        }
        
        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            // The function implementation goes here
        }
    }

    let parser = TestParser::new("a-b");
    let result = parser.parse_set_class_range();
    
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_range_valid_range() {
    struct TestParser {
        input: String,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump_space(&mut self) {
            self.pos.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn peek_space(&self) -> Option<char> {
            if self.pos.offset + 1 < self.input.len() {
                Some(self.input.chars().nth(self.pos.offset + 1).unwrap())
            } else {
                None
            }
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            let c = self.char();
            Ok(Primitive::Literal(Literal {
                span: Span::new(self.pos, self.pos),
                kind: ast::LiteralKind::Verbatim,
                c,
            }))
        }

        fn error(&self, span: &Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: self.input.clone(), span: span.clone() }
        }

        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            // The function implementation goes here
        }
    }

    let parser = TestParser::new("a-z");
    let result = parser.parse_set_class_range();

    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    struct TestParser {
        input: String,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump_space(&mut self) {
            self.pos.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn peek_space(&self) -> Option<char> {
            if self.pos.offset + 1 < self.input.len() {
                Some(self.input.chars().nth(self.pos.offset + 1).unwrap())
            } else {
                None
            }
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            let c = self.char();
            Ok(Primitive::Literal(Literal {
                span: Span::new(self.pos, self.pos),
                kind: ast::LiteralKind::Verbatim,
                c,
            }))
        }

        fn error(&self, span: &Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: self.input.clone(), span: span.clone() }
        }

        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            // The function implementation goes here
        }
    }

    let parser = TestParser::new("z-a");
    let result = parser.parse_set_class_range();

    assert!(result.is_err());
}

