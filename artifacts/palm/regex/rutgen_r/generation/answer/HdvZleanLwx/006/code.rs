// Answer 0

#[test]
fn test_parse_octal_valid_case() {
    struct TestParser {
        octal: bool,
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = TestParser { 
        octal: true, 
        input: "077", 
        pos: 0 
    };
    let literal = parser.parse_octal();

    assert_eq!(literal.kind, ast::LiteralKind::Octal);
    assert_eq!(literal.c, '7');
}

#[test]
#[should_panic]
fn test_parse_octal_over_limit() {
    struct TestParser {
        octal: bool,
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = TestParser { 
        octal: true, 
        input: "088", 
        pos: 0 
    };
    parser.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_character() {
    struct TestParser {
        octal: bool,
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = TestParser { 
        octal: true, 
        input: "07a", 
        pos: 0 
    };
    parser.parse_octal();
}

#[test]
fn test_parse_octal_three_digit_case() {
    struct TestParser {
        octal: bool,
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = TestParser { 
        octal: true, 
        input: "077", 
        pos: 0 
    };
    let literal = parser.parse_octal();

    assert_eq!(literal.kind, ast::LiteralKind::Octal);
    assert_eq!(literal.c, '7');
}

