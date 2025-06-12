// Answer 0

fn test_parse_exponent_positive() -> Result<()> {
    struct MockParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, index: 0 }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn error(&self, code: ErrorCode) -> String {
            format!("{:?}", code)
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            let sign = if positive { 1.0 } else { -1.0 };
            Ok(sign * (significand as f64 * 10f64.powi(exponent)))
        }

        fn parse_exponent_overflow(&self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64> {
            if zero_significand {
                Ok(0.0)
            } else {
                Err(self.error(ErrorCode::InvalidNumber))
            }
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64> {
            // Function implementation provided above...
        }
    }

    let mut parser = MockParser::new(vec![b'e', b'+', b'1', b'2', b'3']);
    let result = parser.parse_exponent(true, 123, 0)?;
    assert_eq!(result, 12300.0);
    Ok(())
}

#[test]
fn test_parse_exponent_negative_exponent() -> Result<()> {
    struct MockParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, index: 0 }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn error(&self, code: ErrorCode) -> String {
            format!("{:?}", code)
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            let sign = if positive { 1.0 } else { -1.0 };
            Ok(sign * (significand as f64 * 10f64.powi(exponent)))
        }

        fn parse_exponent_overflow(&self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64> {
            if zero_significand {
                Ok(0.0)
            } else {
                Err(self.error(ErrorCode::InvalidNumber))
            }
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64> {
            // Function implementation provided above...
        }
    }

    let mut parser = MockParser::new(vec![b'e', b'-', b'1', b'0']);
    let result = parser.parse_exponent(true, 123, 5)?;
    assert_eq!(result, 12.3);
    Ok(())
}

#[test]
fn test_parse_exponent_invalid_character() {
    struct MockParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, index: 0 }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn error(&self, code: ErrorCode) -> String {
            format!("{:?}", code)
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            Ok(0.0) // Placeholder implementation
        }

        fn parse_exponent_overflow(&self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64> {
            Ok(0.0) // Placeholder implementation
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64> {
            // Function implementation provided above...
        }
    }

    let mut parser = MockParser::new(vec![b'e', b'+', b'a']);
    let result = parser.parse_exponent(true, 123, 0);
    assert!(result.is_err());
}

