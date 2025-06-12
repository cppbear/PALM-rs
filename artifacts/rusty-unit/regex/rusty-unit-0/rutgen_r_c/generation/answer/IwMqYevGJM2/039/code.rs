// Answer 0

#[test]
fn test_parse_unicode_class_single_character_notation() {
    struct MockParser {
        pos: Position,
        scratch: RefCell<String>,
        char: char,
    }

    impl MockParser {
        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::__Nonexhaustive,
                pattern: "".to_string(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let mut parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        scratch: RefCell::new(String::from("N")),
        char: 'p',
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::OneLetter('N'));
}

#[test]
fn test_parse_unicode_class_bracketed_notation() {
    struct MockParser {
        pos: Position,
        scratch: RefCell<String>,
        char: char,
    }

    impl MockParser {
        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::__Nonexhaustive,
                pattern: "".to_string(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let mut parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        scratch: RefCell::new(String::from("Greek")),
        char: 'p',
    };

    parser.bump_and_bump_space(); // Simulate 'bump_and_bump_space' being called, moving past 'p'
    // Simulate moving to the next character
    parser.char = '{'; // Set char to '{' for the bracketed notation

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::Named("Greek".to_string()));
}

