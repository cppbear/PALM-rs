// Answer 0

#[test]
fn test_parse_octal_valid_three_digit_octal() {
    struct MockParser {
        octal: bool,
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(octal: bool, input: &str) -> Self {
            Self {
                octal,
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    impl MockParser {
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = MockParser::new(true, "0777abc");
    let result = parser.parse_octal();

    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, 'ǅ'); // Assuming 'ǅ' is the character for octal 777
}

#[test]
#[should_panic(expected = "valid octal number")]
fn test_parse_octal_invalid_four_digit_octal() {
    struct MockParser {
        octal: bool,
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(octal: bool, input: &str) -> Self {
            Self {
                octal,
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    impl MockParser {
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = MockParser::new(true, "0888abc");
    parser.parse_octal(); // This should panic
}

#[test]
fn test_parse_octal_valid_two_digit_octal() {
    struct MockParser {
        octal: bool,
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(octal: bool, input: &str) -> Self {
            Self {
                octal,
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    impl MockParser {
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = MockParser::new(true, "077abc");
    let result = parser.parse_octal();

    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, 'ǅ'); // Assuming 'ǅ' is the character for octal 77
}

