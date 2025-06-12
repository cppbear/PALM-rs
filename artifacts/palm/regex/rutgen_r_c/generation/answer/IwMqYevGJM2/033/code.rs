// Answer 0

#[test]
fn test_parse_unicode_class_panic_escape_unexpected_eof() {
    struct TestParser {
        chars: Vec<char>,
        pos: Position,
        current_char_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            let chars: Vec<char> = pattern.chars().collect();
            TestParser {
                chars,
                pos: Position { offset: 0, line: 1, column: 1 },
                current_char_index: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.current_char_index >= self.chars.len()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.is_eof() {
                return false;
            }
            self.current_char_index += 1;
            true
        }

        fn char(&self) -> char {
            self.chars[self.current_char_index]
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos) // span is at the current position
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test pattern"), span }
        }
    }

    impl<'s> ParserI<'s, TestParser> {
        fn new(parser: TestParser) -> Self {
            ParserI { parser, pattern: "" }
        }
    }

    let mut test_parser = TestParser::new("p");
    let parser = ParserI::new(test_parser);
    
    let result = parser.parse_unicode_class();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::EscapeUnexpectedEof);
    }
}

#[test]
fn test_parse_unicode_class_empty_string() {
    struct TestParser {
        chars: Vec<char>,
        pos: Position,
        current_char_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            let chars: Vec<char> = pattern.chars().collect();
            TestParser {
                chars,
                pos: Position { offset: 0, line: 1, column: 1 },
                current_char_index: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.current_char_index >= self.chars.len()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.is_eof() {
                return false;
            }
            self.current_char_index += 1;
            true
        }

        fn char(&self) -> char {
            self.chars[self.current_char_index]
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos) // span is at the current position
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test pattern"), span }
        }
    }

    impl<'s> ParserI<'s, TestParser> {
        fn new(parser: TestParser) -> Self {
            ParserI { parser, pattern: "" }
        }
    }

    let mut test_parser = TestParser::new("");
    let parser = ParserI::new(test_parser);
    
    let result = parser.parse_unicode_class();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::EscapeUnexpectedEof);
    }
}

