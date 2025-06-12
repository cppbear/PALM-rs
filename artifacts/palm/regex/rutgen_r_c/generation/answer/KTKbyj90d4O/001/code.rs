// Answer 0

#[test]
fn test_parse_capture_name_eof() {
    struct MockParser {
        input: String,
        pos: Position,
        eof: bool,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0' // Simulate EOF character
            } else {
                self.input.chars().nth(self.pos.offset).unwrap_or('\0')
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            !self.is_eof()
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<()> {
            // Simulate success of adding capture name.
            Ok(())
        }
    }

    let parser = MockParser {
        input: String::new(), // Empty input simulates EOF situation
        pos: Position { offset: 0, line: 1, column: 1 },
        eof: true,
    };
    let result = parser.parse_capture_name(0);
    assert!(result.is_err()); // Ensure we get an Err
    if let Err(err) = result {
        // Check for the expected error kind
        assert_eq!(err.kind, ast::ErrorKind::GroupNameUnexpectedEof);
    }
}

