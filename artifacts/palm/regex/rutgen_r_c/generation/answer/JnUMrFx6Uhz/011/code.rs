// Answer 0

#[test]
fn test_parse_flags_with_dangling_negation() {
    struct MockParser {
        chars: Vec<char>,
        pos: Position,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self {
                chars,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos.offset).unwrap_or(&')')
        }

        fn span_char(&self) -> Span {
            Span {
                start: self.pos,
                end: self.pos,
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset + 1 < self.chars.len() {
                self.pos.offset += 1;
                self.pos.column += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: "test".to_string(),
                span: self.span_char(),
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }
    }

    let mut parser = MockParser::new(vec!['-', 'i', ':']);
    let result = parser.parse_flags();
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::FlagDanglingNegation);
    }
}

