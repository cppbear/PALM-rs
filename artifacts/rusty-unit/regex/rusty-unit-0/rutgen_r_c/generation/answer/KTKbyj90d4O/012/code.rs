// Answer 0

#[test]
fn test_parse_capture_name_eof_error() {
    use ast::{CaptureName, ErrorKind};

    struct TestParser {
        pos: Position,
        pattern: String,
        eof: bool,
        char_at_pos: char,
    }

    impl TestParser {
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            self.char_at_pos
        }

        fn bump(&mut self) -> bool {
            // Simulate not moving the position, always returns false
            false
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos) // Return invalid span for simplicity
        }

        fn error(&self, span: Span, kind: ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let test_parser = TestParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<captureName>".to_string(),
        eof: false, // Make sure is_eof() is false
        char_at_pos: '>', // The char should be '>'
    };

    // Act
    let result = test_parser.parse_capture_name(0);

    // Assert
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ErrorKind::GroupNameUnexpectedEof);
    }
}

