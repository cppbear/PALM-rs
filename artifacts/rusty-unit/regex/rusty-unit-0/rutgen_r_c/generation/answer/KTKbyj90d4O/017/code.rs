// Answer 0

#[test]
fn test_parse_capture_name_invalid_character() {
    struct MockParser {
        pattern: String,
        pos: Position,
        eof: bool,
        current_char: char,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                eof: false,
                current_char: '<', // Simulate being at the starting point of a capture name
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                if self.pos.offset < self.pattern.len() {
                    self.current_char = self.pattern[self.pos.offset..].chars().next().unwrap();
                    true
                } else {
                    self.eof = true;
                    false
                }
            } else {
                self.eof = true;
                false
            }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos) // Mock span using current position
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span: self.span_char(),
            }
        }

        fn parse_capture_name(&mut self, capture_index: u32) -> Result<ast::CaptureName> {
            if self.is_eof() {
                return Err(self.error(self.span_char(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            let start = self.pos;
            while self.char() != '>' {
                if !is_capture_char(self.char(), self.pos.offset == start.offset) {
                    return Err(self.error(self.span_char(), ast::ErrorKind::GroupNameInvalid));
                }
                if !self.bump() {
                    break;
                }
            }
            let end = self.pos;
            if self.is_eof() {
                return Err(self.error(self.span_char(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            assert_eq!(self.char(), '>');
            self.bump();
            let name = &self.pattern[start.offset..end.offset];
            if name.is_empty() {
                return Err(self.error(Span::new(start, start), ast::ErrorKind::GroupNameEmpty));
            }
            Ok(ast::CaptureName {
                span: Span::new(start, end),
                name: name.to_string(),
                index: capture_index,
            })
        }
    }
    
    let mut parser = MockParser::new("<1");
    parser.bump(); // Move past '<'
    parser.current_char = '1'; // Simulate an invalid character for capture name
    let result = parser.parse_capture_name(0);
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::GroupNameInvalid);
    }
}

