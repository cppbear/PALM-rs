// Answer 0

#[test]
fn test_parse_capture_name_unexpected_eof() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, pos: 0 }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0' // character at EOF
            } else {
                self.chars[self.pos]
            }
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> Result<ast::CaptureName> {
            Err(kind.into())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn pattern(&self) -> &[char] {
            &self.chars
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = MockParser::new(vec!['<', 'c', 'a', 'p', 't', 'u', 'r', 'e']);
    
    // Setting the parser position to the start after '<'.
    parser.bump();

    // Condition for the test:
    // After reading 'capture', we will make the parser return EOF
    // and hit the '>' condition just before returning an error.
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
    if let Err(kind) = result {
        assert_eq!(kind, ast::ErrorKind::GroupNameUnexpectedEof);
    }
}

