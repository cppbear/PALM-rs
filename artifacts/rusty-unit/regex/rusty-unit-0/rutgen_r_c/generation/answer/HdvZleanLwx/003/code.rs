// Answer 0

#[test]
fn test_parse_octal_valid() {
    struct DummyParser {
        octal: bool,
        chars: Vec<char>,
        position: usize,
    }

    impl DummyParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap()
        }

        fn bump(&mut self) -> bool {
            if self.position + 1 < self.chars.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pattern(&self) -> &str {
            "012345"
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.position,
                line: 1,
                column: self.position + 1,
            }
        }
    }

    let mut parser = DummyParser {
        octal: true,
        chars: vec!['0', '1', '2'],
        position: 0,
    };
    
    let result = parser.parse_octal();
    assert_eq!(result.c, '2');
    assert_eq!(result.kind, LiteralKind::Octal);
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_char() {
    struct DummyParser {
        octal: bool,
        chars: Vec<char>,
        position: usize,
    }

    impl DummyParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap()
        }

        fn bump(&mut self) -> bool {
            if self.position + 1 < self.chars.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pattern(&self) -> &str {
            "08"
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.position,
                line: 1,
                column: self.position + 1,
            }
        }
    }

    let mut parser = DummyParser {
        octal: true,
        chars: vec!['0', '8'],
        position: 0,
    };
    
    parser.parse_octal(); // This will panic since '8' is invalid for octal
}

#[test]
fn test_parse_octal_exceeding_three_digits() {
    struct DummyParser {
        octal: bool,
        chars: Vec<char>,
        position: usize,
    }

    impl DummyParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap()
        }

        fn bump(&mut self) -> bool {
            if self.position + 1 < self.chars.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pattern(&self) -> &str {
            "0777"
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.position,
                line: 1,
                column: self.position + 1,
            }
        }
    }

    let mut parser = DummyParser {
        octal: true,
        chars: vec!['0', '7', '7', '7'],
        position: 0,
    };
    
    let result = parser.parse_octal();
    assert_eq!(result.c, '7');
    assert_eq!(result.kind, LiteralKind::Octal);
}

