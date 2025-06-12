// Answer 0

#[test]
fn test_parse_set_class_open_unclosed_class() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }
    
    impl MockParser {
        fn char(&self) -> char {
            self.input.get(self.pos).copied().unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos += 1;
            }
            let current = self.char();
            if current == '\0' {
                return false;
            }
            self.pos += 1; // Bump
            true
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }
    }

    let mut parser = MockParser { input: vec!['[', ' '], pos: 0 };
    let result = parse_set_class_open(&mut parser);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, ast::ErrorKind::ClassUnclosed);
    }
}

#[test]
fn test_parse_set_class_open_with_empty_class() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.input.get(self.pos).copied().unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos += 1;
            }
            let current = self.char();
            if current == '\0' {
                return false;
            }
            self.pos += 1; // Bump
            true
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }
    }

    let mut parser = MockParser { input: vec!['[', ']', ' '], pos: 0 };
    let result = parse_set_class_open(&mut parser);
    assert!(result.is_ok());
    let (set, union) = result.unwrap();
    assert!(!set.negated);
    assert!(union.items.is_empty());
}

