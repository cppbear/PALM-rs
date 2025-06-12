// Answer 0

#[test]
fn test_parse_integer_leading_zero() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let result = self.input[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Simulated error handling
        }
    }

    let mut parser = TestParser {
        input: vec![b'0'],
        index: 0,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_invalid_leading_digit() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let result = self.input[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, ()> {
            Ok(0) // Simulated success
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Simulated error handling
        }
    }

    let mut parser = TestParser {
        input: vec![b'0', b'1'],
        index: 0,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

