// Answer 0

#[test]
fn test_parse_unicode_class_single_character() {
    struct MockParser {
        chars: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1;
            self.position < self.chars.len()
        }
        
        fn is_eof(&self) -> bool {
            self.position >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> (usize, usize) {
            (self.position, self.position + 1)
        }

        fn error(&self, _span: (usize, usize), _kind: ()) -> Result<ast::ClassUnicode, ()> {
            Err(())
        }
    }

    let mut parser = MockParser {
        chars: vec!['p', 'a'],
        position: 0,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());

    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::OneLetter('a'));
}

#[test]
fn test_parse_unicode_class_empty_name() {
    struct MockParser {
        chars: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1;
            true
        }
        
        fn is_eof(&self) -> bool {
            self.position >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> (usize, usize) {
            (self.position, self.position + 1)
        }

        fn error(&self, _span: (usize, usize), _kind: ()) -> Result<ast::ClassUnicode, ()> {
            Err(())
        }
    }

    let mut parser = MockParser {
        chars: vec!['p', '{', '}', 'a'],
        position: 0,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::Named("".to_string()));
}

#[test]
fn test_parse_unicode_class_named_value() {
    struct MockParser {
        chars: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1;
            self.position < self.chars.len()
        }
        
        fn is_eof(&self) -> bool {
            self.position >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> (usize, usize) {
            (self.position, self.position + 1)
        }

        fn error(&self, _span: (usize, usize), _kind: ()) -> Result<ast::ClassUnicode, ()> {
            Err(())
        }
    }

    let mut parser = MockParser {
        chars: vec!['p', '{', 'N', 'a', 'm', 'e', '!', '=', 'v', 'a', 'l', 'u', 'e', '}'],
        position: 0,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());

    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::NamedValue {
        op: ast::ClassUnicodeOpKind::NotEqual,
        name: "Name".to_string(),
        value: "value".to_string(),
    });
}

#[test]
fn test_parse_unicode_class_colon_value() {
    struct MockParser {
        chars: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1;
            self.position < self.chars.len()
        }
        
        fn is_eof(&self) -> bool {
            self.position >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> (usize, usize) {
            (self.position, self.position + 1)
        }

        fn error(&self, _span: (usize, usize), _kind: ()) -> Result<ast::ClassUnicode, ()> {
            Err(())
        }
    }

    let mut parser = MockParser {
        chars: vec!['p', '{', 'N', 'a', 'm', 'e', ':', 'v', 'a', 'l', 'u', 'e', '}'],
        position: 0,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());

    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::NamedValue {
        op: ast::ClassUnicodeOpKind::Colon,
        name: "Name".to_string(),
        value: "value".to_string(),
    });
}

#[test]
fn test_parse_unicode_class_equal_value() {
    struct MockParser {
        chars: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1;
            self.position < self.chars.len()
        }
        
        fn is_eof(&self) -> bool {
            self.position >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> (usize, usize) {
            (self.position, self.position + 1)
        }

        fn error(&self, _span: (usize, usize), _kind: ()) -> Result<ast::ClassUnicode, ()> {
            Err(())
        }
    }

    let mut parser = MockParser {
        chars: vec!['p', '{', 'N', 'a', 'm', 'e', '=', 'v', 'a', 'l', 'u', 'e', '}'],
        position: 0,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());

    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::NamedValue {
        op: ast::ClassUnicodeOpKind::Equal,
        name: "Name".to_string(),
        value: "value".to_string(),
    });
}

