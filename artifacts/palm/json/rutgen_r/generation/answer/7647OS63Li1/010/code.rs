// Answer 0

#[test]
fn test_parse_integer_with_zero() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, significand: u64) -> Result<ParserNumber, &'static str> {
            Ok(ParserNumber::U64(significand))
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "error occurred"
        }
    }

    let mut parser = TestParser { input: vec![b'0'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(0)));
}

#[test]
fn test_parse_integer_with_valid_digit() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, significand: u64) -> Result<ParserNumber, &'static str> {
            Ok(ParserNumber::U64(significand))
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "error occurred"
        }

        fn parse_long_integer(&self, _positive: bool, significand: u64) -> Result<u64, &'static str> {
            Ok(significand)
        }
    }

    let mut parser = TestParser { input: vec![b'5', b'3', b'2'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(532)));
}

#[test]
fn test_parse_integer_with_overflow() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, _significand: u64) -> Result<ParserNumber, &'static str> {
            Ok(ParserNumber::U64(u64::MAX))
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "error occurred"
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, &'static str> {
            Ok(u64::MAX)
        }
    }

    let mut parser = TestParser { input: vec![b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'], index: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_ok());
}

#[test]
fn test_parse_integer_with_invalid_char() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "invalid number"
        }
    }

    let mut parser = TestParser { input: vec![b'a'], index: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

