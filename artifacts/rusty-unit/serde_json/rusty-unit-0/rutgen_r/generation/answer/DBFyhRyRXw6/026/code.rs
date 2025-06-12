// Answer 0

#[test]
fn test_parse_exponent_negative_sign() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error::new("Error occurred")
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _final_exp: i32) -> Result<f64> {
            Ok(1.0) // Placeholder for actual implementation
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64> {
            Err(self.error(ErrorCode::InvalidNumber))
        }
    }

    let mut parser = MockParser::new(vec![b'e', b'-', b'1']);
    let result = parser.parse_exponent(true, 5, 2);
    assert_eq!(result, Err(ErrorCode::InvalidNumber.into()));
}

#[test]
fn test_parse_exponent_invalid_digit() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error::new("Error occurred")
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _final_exp: i32) -> Result<f64> {
            Ok(1.0) // Placeholder for actual implementation
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64> {
            Err(self.error(ErrorCode::InvalidNumber))
        }
    }

    let mut parser = MockParser::new(vec![b'e', b'+', b'a']); // Invalid digit 'a'
    let result = parser.parse_exponent(true, 5, 2);
    assert_eq!(result, Err(ErrorCode::InvalidNumber.into()));
}

