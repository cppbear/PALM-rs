// Answer 0

#[test]
fn test_parse_escape_punctuation() {
    struct TestParser<'s> {
        pattern: &'s str,
        pos: Position,
        ignore_whitespace: bool,
    }

    impl<'s> TestParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
                ignore_whitespace: false,
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                self.pos.column += 1; // simplifying for this example
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn error(&self, _: Span, _: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.to_string(),
                span: Span::new(self.pos, self.pos),
            }
        }
        
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let pattern = r"\*"; // testing with '*' as a meta character
    let parser = TestParser::new(pattern);
    let result = parser.parse_escape();

    assert!(result.is_ok());
    match result {
        Ok(Primitive::Literal(lit)) => {
            assert_eq!(lit.kind, ast::LiteralKind::Punctuation);
            assert_eq!(lit.c, '*');
        }
        _ => panic!("Expected a Primitive::Literal"),
    }
}

#[test]
fn test_parse_escape_special() {
    struct TestParser<'s> {
        pattern: &'s str,
        pos: Position,
    }

    impl<'s> TestParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                self.pos.column += 1; // simplifying for this example
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn error(&self, _: Span, _: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.to_string(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let pattern = r"\n"; // testing with '\n' for line feed
    let parser = TestParser::new(pattern);
    let result = parser.parse_escape();

    assert!(result.is_ok());
    match result {
        Ok(Primitive::Literal(lit)) => {
            assert_eq!(lit.kind, ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed));
            assert_eq!(lit.c, '\n');
        }
        _ => panic!("Expected a Primitive::Literal"),
    }
}

#[test]
fn test_parse_escape_assertion() {
    struct TestParser<'s> {
        pattern: &'s str,
        pos: Position,
    }

    impl<'s> TestParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                self.pos.column += 1; // simplifying for this example
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn error(&self, _: Span, _: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.to_string(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let pattern = r"\b"; // testing word boundary assertion
    let parser = TestParser::new(pattern);
    let result = parser.parse_escape();

    assert!(result.is_ok());
    match result {
        Ok(Primitive::Assertion(assertion)) => {
            assert_eq!(assertion.kind, ast::AssertionKind::WordBoundary);
        }
        _ => panic!("Expected a Primitive::Assertion"),
    }
}

