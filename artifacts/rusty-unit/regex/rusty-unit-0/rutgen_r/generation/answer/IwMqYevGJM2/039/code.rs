// Answer 0

#[test]
fn test_parse_unicode_class_named() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }
    
    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            if self.pos < self.chars.len() {
                self.pos += 1;
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate bumping, skipping spaces if they are there
            while self.char().is_whitespace() {
                self.bump();
            }
            self.char() != '\0'
        }

        fn pos(&self) -> usize {
            self.pos
        }
        
        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }
    }

    struct Mock {
        parser: MockParser,
    }

    impl Mock {
        fn char(&self) -> char {
            self.parser.char()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.parser.bump_and_bump_space()
        }

        fn pos(&self) -> usize {
            self.parser.pos()
        }

        fn is_eof(&self) -> bool {
            self.parser.is_eof()
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::ClassUnicode {
            ast::ClassUnicode {
                span: Span::new(0, 0),
                negated: false,
                kind: ast::ClassUnicodeKind::Named("error".to_string()),
            }
        }
    }

    let chars = vec!['p', '{', 'G', 'r', 'e', 'e', 'k', '}', ' '];
    let mut mock = Mock { parser: MockParser { chars, pos: 0 } };

    // Trigger the method under test
    let result = mock.parse_unicode_class();

    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::Named("Greek".to_string()));
}

