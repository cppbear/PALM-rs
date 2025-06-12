// Answer 0

#[test]
fn test_parse_octal_three_digits() {
    struct MockParser {
        position: usize,
        input: &'static str,
        octal: bool,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self {
                position: 0,
                input,
                octal: true,
            }
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = MockParser::new("0777");
    let result = parse_octal(&parser);
    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, 'ÿ'); // Unicode character for octal 0777
}

#[test]
fn test_parse_octal_two_digits() {
    struct MockParser {
        position: usize,
        input: &'static str,
        octal: bool,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self {
                position: 0,
                input,
                octal: true,
            }
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = MockParser::new("07");
    let result = parse_octal(&parser);
    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, ''); // Unicode character for octal 07
}

#[test]
fn test_parse_octal_one_digit() {
    struct MockParser {
        position: usize,
        input: &'static str,
        octal: bool,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self {
                position: 0,
                input,
                octal: true,
            }
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = MockParser::new("0");
    let result = parse_octal(&parser);
    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, '\0'); // Unicode character for octal 0
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_start() {
    struct MockParser {
        position: usize,
        input: &'static str,
        octal: bool,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self {
                position: 0,
                input,
                octal: true,
            }
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &str {
            self.input
        }
    }

    let mut parser = MockParser::new("8");
    let _ = parse_octal(&parser); // Should panic because '8' is not a valid octal digit
}

