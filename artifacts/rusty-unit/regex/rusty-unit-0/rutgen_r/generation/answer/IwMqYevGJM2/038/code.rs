// Answer 0

#[test]
fn test_parse_unicode_class_single_letter() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, pos: 0 }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.chars.len() // Always true in this scenario
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }
    }

    let mut parser = MockParser::new(vec!['p', 'a']);
    let result = parse_unicode_class(&mut parser);
    assert!(result.is_ok());

    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::OneLetter('a'));
}

#[test]
fn test_parse_unicode_class_named_value() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
        scratch: Vec<char>,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, pos: 0, scratch: Vec::new() }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }
    }

    let mut parser = MockParser::new(vec!['p', '{', 'N', 'a', 'm', 'e', '=', 'V', 'a', 'l', 'u', 'e', '}']);
    let result = parse_unicode_class(&mut parser);
    assert!(result.is_ok());

    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::NamedValue {
        op: ast::ClassUnicodeOpKind::Equal,
        name: "Name".to_string(),
        value: "Value".to_string(),
    });
}

