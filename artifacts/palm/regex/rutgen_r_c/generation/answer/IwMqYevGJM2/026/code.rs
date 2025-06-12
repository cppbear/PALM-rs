// Answer 0

#[test]
fn test_parse_unicode_class_single() {
    struct MockParser {}

    impl MockParser {
        fn char(&self) -> char {
            'p'
        }

        fn bump_and_bump_space(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = MockParser {};
    let result = parser.parse_unicode_class().unwrap();

    assert_eq!(result.negated, false);
    assert_eq!(result.kind, ast::ClassUnicodeKind::OneLetter('p'));
}

#[test]
fn test_parse_unicode_class_bracketed() {
    struct MockParser {
        chars: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            MockParser { chars, index: 0 }
        }

        fn char(&self) -> char {
            self.chars[self.index]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.index >= self.chars.len()
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    // Test with bracketed Unicode class
    let chars = vec!['p', '{', 'G', 'r', 'e', 'e', 'k', '}', ' ']; // Includes space after closing brace
    let mut parser = MockParser::new(chars);
    let result = parser.parse_unicode_class().unwrap();

    assert_eq!(result.negated, false);
    if let ast::ClassUnicodeKind::Named(name) = result.kind {
        assert_eq!(name, "Greek");
    } else {
        panic!("Expected Named variant");
    }
}

#[test]
fn test_parse_unicode_class_negative() {
    struct MockParser {}

    impl MockParser {
        fn char(&self) -> char {
            'P'
        }

        fn bump_and_bump_space(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = MockParser {};
    let result = parser.parse_unicode_class().unwrap();

    assert_eq!(result.negated, true);
}

#[test]
#[should_panic]
fn test_parse_unicode_class_eof_before_closing() {
    struct MockParser {
        index: usize,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser { index: 0 }
        }

        fn char(&self) -> char {
            'p' // will trigger 'p'
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.index == 0 {
                self.index += 1; // bumps first character
                true
            } else {
                false // no more characters to bump
            }
        }

        fn is_eof(&self) -> bool {
            true // simulate EOF before expected closing brace
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = MockParser::new();
    parser.parse_unicode_class().unwrap(); // This should panic due to EOF
}

