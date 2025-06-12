// Answer 0

#[test]
fn test_parse_escape_octal_not_supported() {
    struct TestParser {
        pattern: String,
        octal: bool,
        pos: Position,
    }

    impl TestParser {
        fn new(pattern: &str, octal: bool) -> Self {
            Self {
                pattern: pattern.to_string(),
                octal,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }
        
        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or_default()
        }

        fn parser(&self) -> &TestParser {
            self
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }
    }

    let mut test_parser = TestParser::new("\\0", false); // Testing with octal '0'
    assert_eq!(test_parser.bump(), true); // Ensuring bump is true
    let result = test_parser.parse_escape();
    let expected_error = ast::ErrorKind::UnsupportedBackreference;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, expected_error);
}

