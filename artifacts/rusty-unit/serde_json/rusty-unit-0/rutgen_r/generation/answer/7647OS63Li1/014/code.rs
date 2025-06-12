// Answer 0

#[test]
fn test_parse_integer_leading_zero_with_invalid_following() {
    struct TestParser {
        input: Vec<u8>,
        idx: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.idx < self.input.len() {
                let c = self.input[self.idx];
                self.idx += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.idx < self.input.len() {
                Ok(self.input[self.idx])
            } else {
                Ok(0) // Simulate a null
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Mock error handling
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, ()> {
            Ok(1) // Mock successful long integer parsing
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<ParserNumber, ()> {
            Err(()) // Mock error
        }
    }

    let mut parser = TestParser {
        input: b"00".to_vec(), // Leading zero followed by another zero
        idx: 0,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_valid_input() {
    struct TestParser {
        input: Vec<u8>,
        idx: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.idx < self.input.len() {
                let c = self.input[self.idx];
                self.idx += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.idx < self.input.len() {
                Ok(self.input[self.idx])
            } else {
                Ok(0) // Simulate a null
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Mock error handling
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, ()> {
            Ok(1) // Mock successful long integer parsing
        }

        fn eat_char(&mut self) {
            self.idx += 1; // mock eating a character
        }
    }

    let mut parser = TestParser {
        input: b"123".to_vec(),
        idx: 0,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_ok());
}

#[test]
fn test_parse_integer_invalid_input() {
    struct TestParser {
        input: Vec<u8>,
        idx: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.idx < self.input.len() {
                let c = self.input[self.idx];
                self.idx += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.idx < self.input.len() {
                Ok(self.input[self.idx])
            } else {
                Ok(0) // Simulate a null
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Mock error handling
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, ()> {
            Ok(1) // Mock successful long integer parsing
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<ParserNumber, ()> {
            Err(()) // Mock error
        }
    }

    let mut parser = TestParser {
        input: b"xyz".to_vec(), // Invalid input
        idx: 0,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

