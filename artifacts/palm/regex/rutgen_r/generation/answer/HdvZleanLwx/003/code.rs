// Answer 0

#[test]
fn test_parse_octal_valid_input() {
    struct MockParser {
        octal: bool,
        input: Vec<char>,
        position: usize,
    }
    
    impl MockParser {
        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &[char] {
            &self.input
        }
    }

    let input = vec!['0', '7', '5'];
    let mut parser = MockParser {
        octal: true,
        input,
        position: 0,
    };

    let result = parser.parse_octal();
    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, '7');
}

#[test]
#[should_panic(expected = "valid octal number")]
fn test_parse_octal_invalid_input() {
    struct MockParser {
        octal: bool,
        input: Vec<char>,
        position: usize,
    }
    
    impl MockParser {
        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &[char] {
            &self.input
        }
    }

    let input = vec!['0', '8']; // '8' is invalid in octal
    let mut parser = MockParser {
        octal: true,
        input,
        position: 0,
    };

    parser.parse_octal();
}

#[test]
#[should_panic(expected = "Unicode scalar value")]
fn test_parse_octal_out_of_bounds() {
    struct MockParser {
        octal: bool,
        input: Vec<char>,
        position: usize,
    }
    
    impl MockParser {
        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &[char] {
            &self.input
        }
    }

    let input = vec!['0', '7', '7']; // Maximum is '0777'
    let mut parser = MockParser {
        octal: true,
        input,
        position: 0,
    };

    let result = parser.parse_octal();
    assert_eq!(result.c, '7'); // Should be '7' initially
    // Triggering panic
    parser.position += 1; // Move beyond valid input
    parser.parse_octal(); // This should panic
}

