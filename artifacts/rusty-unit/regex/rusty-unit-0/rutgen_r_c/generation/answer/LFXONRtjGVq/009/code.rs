// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_range() {
    struct MockParser {
        pos: Position,
        chars: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            MockParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                chars,
                index: 0,
            }
        }

        fn char(&self) -> char {
            self.chars[self.index]
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn parse_decimal(&self) -> Result<u32> {
            Ok(2)  // Arbitrary valid return for testing
        }

        fn is_eof(&self) -> bool {
            self.index >= self.chars.len()
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos) // Simplification for testing
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("Test"), span }
        }
    }

    let parser = MockParser::new(vec!['{', ' ', '1', ',', '2', ' ', '?', '}']);
    
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }))],
    };

    let result = parser.parse_counted_repetition(concat);

    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::RepetitionCountInvalid);
    }
}

