// Answer 0

#[test]
#[should_panic]
fn test_parse_octal_invalid_char_too_low() {
    struct TestParser {
        char_index: usize,
        chars: Vec<char>,
    }

    impl TestParser {
        fn octal(&self) -> bool {
            true
        }

        fn char(&self) -> char {
            self.chars[self.char_index]
        }

        fn bump(&mut self) -> bool {
            self.char_index += 1;
            self.char_index < self.chars.len()
        }

        fn pos(&self) -> usize {
            self.char_index
        }

        fn pattern(&self) -> Vec<char> {
            self.chars.clone()
        }
    }

    let mut parser = TestParser {
        char_index: 0,
        chars: vec!['-'], // Character is below '0'
    };
    
    let _ = parse_octal(&mut parser); // This should panic since it violates '0' <= char
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_char_too_high() {
    struct TestParser {
        char_index: usize,
        chars: Vec<char>,
    }

    impl TestParser {
        fn octal(&self) -> bool {
            true
        }

        fn char(&self) -> char {
            self.chars[self.char_index]
        }

        fn bump(&mut self) -> bool {
            self.char_index += 1;
            self.char_index < self.chars.len()
        }

        fn pos(&self) -> usize {
            self.char_index
        }

        fn pattern(&self) -> Vec<char> {
            self.chars.clone()
        }
    }

    let mut parser = TestParser {
        char_index: 0,
        chars: vec!['8'], // Character is above '7'
    };
    
    let _ = parse_octal(&mut parser); // This should panic since it violates '0' <= char
}

