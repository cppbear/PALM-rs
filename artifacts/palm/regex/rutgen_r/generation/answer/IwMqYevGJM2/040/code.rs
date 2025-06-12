// Answer 0

#[test]
fn test_parse_unicode_class_escape_unexpected_eof() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<ast::ClassUnicode, ast::Error> {
            Err(ast::Error::new(span, kind))
        }

        fn span(&self) -> Span {
            Span::new(self.position, self.position)
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

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<ast::ClassUnicode, ast::Error> {
            self.parser.error(span, kind)
        }

        fn span(&self) -> Span {
            self.parser.span()
        }

        fn parse_unicode_class(&mut self) -> Result<ast::ClassUnicode> {
            assert!(self.char() == 'p' || self.char() == 'P');

            let mut scratch = self.parser.scratch.borrow_mut();
            scratch.clear();

            let negated = self.char() == 'P';
            if !self.bump_and_bump_space() {
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }

            // Simulate the parsing logic from the original function
            // Here we only care about this branch causing error due to EOF
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }

            Ok(ast::ClassUnicode {
                span: self.span(),
                negated: negated,
                kind: ast::ClassUnicodeKind::OneLetter(self.char()),
            })
        }
    }

    // Setup for the test case: character is 'p' and we are returning an Err
    let parser = MockParser {
        input: vec!['p'], // starting char is 'p'
        position: 0,
    };
    let mut mock = Mock { parser };

    let result = mock.parse_unicode_class();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ast::ErrorKind::EscapeUnexpectedEof);
}

