// Answer 0

#[test]
fn test_parse_unicode_class_one_letter() {
    struct FakeParser {
        current_char: char,
        position: Position,
        reached_eof: bool,
        scratch: String,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                current_char: 'p',
                position: Position { offset: 0, line: 1, column: 1 },
                reached_eof: false,
                scratch: String::new(),
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position.offset += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.reached_eof
        }

        fn pos(&self) -> usize {
            self.position.offset
        }

        fn span_char(&self) -> Span {
            Span::new(self.position.clone(), self.position.clone())
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: "error".to_string(), span }
        }
    }

    let parser = FakeParser::new();
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    match class_unicode.kind {
        ast::ClassUnicodeKind::OneLetter(c) => assert_eq!(c, 'p'),
        _ => panic!("Expected OneLetter kind"),
    }
}

#[test]
fn test_parse_unicode_class_named_value() {
    struct FakeParser {
        current_char: char,
        position: Position,
        reached_eof: bool,
        scratch: String,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                current_char: 'p',
                position: Position { offset: 0, line: 1, column: 1 },
                reached_eof: false,
                scratch: "scx=Greek".to_string(),
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position.offset += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.reached_eof
        }

        fn pos(&self) -> usize {
            self.position.offset
        }

        fn span_char(&self) -> Span {
            Span::new(self.position.clone(), self.position.clone())
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: "error".to_string(), span }
        }
    }

    let parser = FakeParser::new();
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    match class_unicode.kind {
        ast::ClassUnicodeKind::NamedValue { op, name, value } => {
            assert_eq!(op, ast::ClassUnicodeOpKind::Equal);
            assert_eq!(name, "scx");
            assert_eq!(value, "Greek");
        }
        _ => panic!("Expected NamedValue kind"),
    }
}

#[test]
fn test_parse_unicode_class_named() {
    struct FakeParser {
        current_char: char,
        position: Position,
        reached_eof: bool,
        scratch: String,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                current_char: 'p',
                position: Position { offset: 0, line: 1, column: 1 },
                reached_eof: false,
                scratch: "Greek".to_string(),
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position.offset += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.reached_eof
        }

        fn pos(&self) -> usize {
            self.position.offset
        }

        fn span_char(&self) -> Span {
            Span::new(self.position.clone(), self.position.clone())
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: "error".to_string(), span }
        }
    }

    let parser = FakeParser::new();
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    match class_unicode.kind {
        ast::ClassUnicodeKind::Named(name) => assert_eq!(name, "Greek"),
        _ => panic!("Expected Named kind"),
    }
}

#[test]
fn test_parse_unicode_class_negated() {
    struct FakeParser {
        current_char: char,
        position: Position,
        reached_eof: bool,
        scratch: String,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                current_char: 'P',
                position: Position { offset: 0, line: 1, column: 1 },
                reached_eof: false,
                scratch: "scx!=Greek".to_string(),
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position.offset += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.reached_eof
        }

        fn pos(&self) -> usize {
            self.position.offset
        }

        fn span_char(&self) -> Span {
            Span::new(self.position.clone(), self.position.clone())
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: "error".to_string(), span }
        }
    }

    let parser = FakeParser::new();
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, true);
    match class_unicode.kind {
        ast::ClassUnicodeKind::NamedValue { op, name, value } => {
            assert_eq!(op, ast::ClassUnicodeOpKind::NotEqual);
            assert_eq!(name, "scx");
            assert_eq!(value, "Greek");
        }
        _ => panic!("Expected NamedValue kind"),
    }
}

