// Answer 0

#[test]
fn test_parse_set_class_range_unclosed_class_error() {
    struct MockParser {
        cursor: usize,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                cursor: 0,
                pattern: pattern.to_string(),
            }
        }

        fn is_eof(&self) -> bool {
            self.cursor >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.cursor).unwrap_or('\0')
        }

        fn bump(&mut self) {
            if self.cursor < self.pattern.len() {
                self.cursor += 1;
            }
        }

        fn bump_space(&mut self) -> bool {
            self.bump(); // Simulate bumping past the character
            true // Always return true for this mock
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            Ok(Primitive::Literal(ast::Literal {
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
                kind: ast::LiteralKind::Verbatim,
                c: 'a',
            }))
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::ClassUnclosed,
                pattern: self.pattern.clone(),
                span: Span::new(Position { offset: self.cursor, line: 1, column: 1 }, Position { offset: self.cursor, line: 1, column: 1 }),
            }
        }
    }

    let parser = MockParser::new("a-"); // Here we expect "a" should be parsed as a single item, but the class is unclosed.
    
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ast::ErrorKind::ClassUnclosed);
}

