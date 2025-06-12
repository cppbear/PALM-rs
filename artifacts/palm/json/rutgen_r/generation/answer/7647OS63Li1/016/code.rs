// Answer 0

#[test]
fn test_parse_integer_valid_single_digit() {
    struct Parser {
        input: Vec<u8>,
        pos: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.pos < self.input.len() {
                let result = self.input[self.pos];
                self.pos += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ErrorCode> {
            // Stub implementation for the test
            Ok(ParserNumber::U64(if positive { significand } else { significand * -1 }))
        }

        fn parse_long_integer(&self, _positive: bool, significand: u64) -> Result<f64, ErrorCode> {
            // Stub implementation for the test
            Ok(significand as f64)
        }

        fn error(&self, code: ErrorCode) -> ErrorCode {
            code
        }
    }

    let mut parser = Parser {
        input: vec![b'1', b'2', b'3'],
        pos: 0,
    };

    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(1)));
}

#[test]
fn test_parse_integer_leading_zero() {
    struct Parser {
        input: Vec<u8>,
        pos: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.pos < self.input.len() {
                let result = self.input[self.pos];
                self.pos += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ErrorCode> {
            // Stub implementation for the test
            Ok(ParserNumber::U64(if positive { significand } else { significand * -1 }))
        }

        fn parse_long_integer(&self, _positive: bool, significand: u64) -> Result<f64, ErrorCode> {
            // Stub implementation for the test
            Ok(significand as f64)
        }

        fn error(&self, code: ErrorCode) -> ErrorCode {
            code
        }
    }

    let mut parser = Parser {
        input: vec![b'0', b'0'],
        pos: 0,
    };

    let result = parser.parse_integer(true);
    assert_eq!(result, Err(ErrorCode::InvalidNumber));
}

#[test]
fn test_parse_integer_overflow() {
    struct Parser {
        input: Vec<u8>,
        pos: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.pos < self.input.len() {
                let result = self.input[self.pos];
                self.pos += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ErrorCode> {
            Ok(ParserNumber::U64(if positive { significand } else { significand * -1 }))
        }

        fn parse_long_integer(&self, _positive: bool, significand: u64) -> Result<f64, ErrorCode> {
            Ok(significand as f64)
        }

        fn error(&self, code: ErrorCode) -> ErrorCode {
            code
        }
    }

    let mut parser = Parser {
        input: vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0'],
        pos: 0,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_ok());
}

