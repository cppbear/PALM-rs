// Answer 0

#[test]
fn test_parse_escape_assertion_word_boundary() {
    struct MockParser {
        pattern: String,
    }
    
    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser { pattern: pattern.to_string() }
        }
        
        fn char(&self) -> char {
            self.pattern.chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            self.pattern = self.pattern.chars().skip(1).collect();
            !self.pattern.is_empty()
        }

        fn pos(&self) -> Position {
            Position { offset: 0, line: 1, column: 1 }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }
    }
    
    let mut parser = MockParser::new("\\b");
    parser.bump(); // Move past the '\'
    
    // Test for Ok(Primitive::Assertion(ast::Assertion { span, kind: ast::AssertionKind::WordBoundary }))
    let result = parser.parse_escape();
    
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, ast::AssertionKind::WordBoundary);
    }
}

