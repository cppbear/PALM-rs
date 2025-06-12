// Answer 0

#[test]
fn test_parse_octal_valid() {
    struct DummyParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
                octal: true,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> Vec<char> {
            self.input.clone()
        }
    }

    let mut parser = DummyParser::new("075");
    let literal = parser.parse_octal();
    assert_eq!(literal.c, '7');
}

#[test]
fn test_parse_octal_empty() {
    struct DummyParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
                octal: true,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> Vec<char> {
            self.input.clone()
        }
    }

    let mut parser = DummyParser::new("0");
    let literal = parser.parse_octal();
    assert_eq!(literal.c, '0');
}

#[test]
fn test_parse_octal_over_three_digits() {
    struct DummyParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
                octal: true,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> Vec<char> {
            self.input.clone()
        }
    }

    let mut parser = DummyParser::new("0789");
    let literal = parser.parse_octal();
    assert_eq!(literal.c, '7');
}

