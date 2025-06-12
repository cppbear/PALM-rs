// Answer 0

#[test]
fn test_parse_unicode_class_single_character() {
    struct DummyParser {
        input: &'static str,
        pos: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: self.pos, line: 1, column: 1 }, Position { offset: self.pos + 1, line: 1, column: 2 })
        }
    }

    let parser = ParserI {
        parser: DummyParser { input: "\\pN", pos: 2 },
        pattern: "\\pN",
    };

    let result = parser.parse_unicode_class();

    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    match class_unicode.kind {
        ast::ClassUnicodeKind::OneLetter(c) => assert_eq!(c, 'N'),
        _ => panic!("Expected OneLetter variant"),
    }
}

#[test]
fn test_parse_unicode_class_bracketed_notation() {
    struct DummyParser {
        input: &'static str,
        pos: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: self.pos, line: 1, column: 1 }, Position { offset: self.pos + 1, line: 1, column: 2 })
        }
    }

    let parser = ParserI {
        parser: DummyParser { input: "\\p{Greek}", pos: 2 },
        pattern: "\\p{Greek}",
    };

    let result = parser.parse_unicode_class();

    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    match class_unicode.kind {
        ast::ClassUnicodeKind::Named(ref name) => assert_eq!(name, "Greek"),
        _ => panic!("Expected Named variant"),
    }
}

#[test]
fn test_parse_unicode_class_negated() {
    struct DummyParser {
        input: &'static str,
        pos: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: self.pos, line: 1, column: 1 }, Position { offset: self.pos + 1, line: 1, column: 2 })
        }
    }

    let parser = ParserI {
        parser: DummyParser { input: "\\P{Latin}", pos: 2 },
        pattern: "\\P{Latin}",
    };

    let result = parser.parse_unicode_class();

    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, true);
    match class_unicode.kind {
        ast::ClassUnicodeKind::Named(ref name) => assert_eq!(name, "Latin"),
        _ => panic!("Expected Named variant"),
    }
}

#[test]
fn test_parse_unicode_class_eof_error() {
    struct DummyParser {
        input: &'static str,
        pos: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: self.pos, line: 1, column: 1 }, Position { offset: self.pos + 1, line: 1, column: 2 })
        }
    }

    let parser = ParserI {
        parser: DummyParser { input: "\\p{", pos: 2 },
        pattern: "\\p{",
    };

    let result = parser.parse_unicode_class();

    assert!(result.is_err());
}

