// Answer 0

fn test_ignore_decimal_no_digits() {
    struct MockParser {
        chars: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, pos: 0 }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0) // Null byte
            }
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            panic!("Invalid number")
        }
    }

    let mut parser = MockParser::new(vec![b'a', b'b', b'c']);
    let result = parser.ignore_decimal();
    assert!(result.is_err());
}

fn test_ignore_decimal_with_digits() {
    struct MockParser {
        chars: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, pos: 0 }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0) // Null byte
            }
        }

        fn ignore_exponent(&mut self) {
            self.eat_char(); // Example implementation
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            panic!("Invalid number")
        }
    }

    let mut parser = MockParser::new(vec![b'1', b'2', b'e', b'3', b'4']);
    let result = parser.ignore_decimal();
    assert!(result.is_ok());
}

fn test_ignore_decimal_only_zero() {
    struct MockParser {
        chars: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, pos: 0 }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0) // Null byte
            }
        }

        fn ignore_exponent(&mut self) {
            self.eat_char(); // Example implementation
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            panic!("Invalid number")
        }
    }

    let mut parser = MockParser::new(vec![b'0', b'e', b'2']);
    let result = parser.ignore_decimal();
    assert!(result.is_ok());
}

