// Answer 0

#[test]
fn test_parse_unicode_class_single_letter() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1; // move past the current character
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("p a");
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_class_named_value_not_equal() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1; // move past current character
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("p {a != b}");
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    match class_unicode.kind {
        ast::ClassUnicodeKind::NamedValue { op, name, value } => {
            assert_eq!(op, ast::ClassUnicodeOpKind::NotEqual);
            assert_eq!(name, "a");
            assert_eq!(value, "b");
        },
        _ => panic!("Expected NamedValue kind."),
    }
}

