// Answer 0

#[test]
fn test_parse_octal_valid_case() {
    struct FakeParser {
        octal: bool,
        input: Vec<char>,
        pos: usize,
    }

    impl FakeParser {
        fn new(input: &str) -> Self {
            Self {
                octal: true,
                input: input.chars().collect(),
                pos: 0,
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

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    let mut parser = FakeParser::new("075");
    let literal = parse_octal(&mut parser);
    assert_eq!(literal.kind, ast::LiteralKind::Octal);
    assert_eq!(literal.c, '\u{003F}'); // '?', which is the result of parsing octal "075"
}

#[test]
#[should_panic(expected = "valid octal number")]
fn test_parse_octal_invalid_number() {
    struct FakeParser {
        octal: bool,
        input: Vec<char>,
        pos: usize,
    }

    impl FakeParser {
        fn new(input: &str) -> Self {
            Self {
                octal: true,
                input: input.chars().collect(),
                pos: 0,
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

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    let mut parser = FakeParser::new("08");
    parse_octal(&mut parser); // Should panic due to invalid octal digit '8'
}

#[test]
#[should_panic(expected = "Unicode scalar value")]
fn test_parse_octal_out_of_range() {
    struct FakeParser {
        octal: bool,
        input: Vec<char>,
        pos: usize,
    }

    impl FakeParser {
        fn new(input: &str) -> Self {
            Self {
                octal: true,
                input: input.chars().collect(),
                pos: 0,
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

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    let mut parser = FakeParser::new("377"); // This exceeds the valid Unicode scalar range
    parse_octal(&mut parser); // Should panic due to out of range codepoint
}

