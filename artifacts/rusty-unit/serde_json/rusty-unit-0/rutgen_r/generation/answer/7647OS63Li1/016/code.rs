// Answer 0

#[test]
fn test_parse_integer_leading_zero() {
    struct MockParser {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.chars.len() {
                let c = self.chars[self.position];
                self.position += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, _significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(0)) // Mocked implementation
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Mock error function
        }
    }

    let mut parser = MockParser { chars: vec![b'0'], position: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_ok());
}

#[test]
fn test_parse_integer_non_zero() {
    struct MockParser {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.chars.len() {
                let c = self.chars[self.position];
                self.position += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, _significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(1)) // Mock implementation
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<f64, ()> {
            Ok(1.0) // Mock implementation
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Mock error function
        }
    }

    let mut parser = MockParser { chars: vec![b'1', b'2', b'3'], position: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_ok());
}

#[test]
fn test_parse_integer_invalid_character() {
    struct MockParser {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.chars.len() {
                let c = self.chars[self.position];
                self.position += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, _significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(0)) // Mock implementation
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Mock error function
        }
    }

    let mut parser = MockParser { chars: vec![b'0', b'1'], position: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_eof() {
    struct MockParser {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.chars.len() {
                let c = self.chars[self.position];
                self.position += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            Ok(0) // Always return null to trigger EOF
        }

        fn parse_number(&self, _positive: bool, _significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(0)) // Mock implementation
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Mock error function
        }
    }

    let mut parser = MockParser { chars: vec![b'1'], position: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

