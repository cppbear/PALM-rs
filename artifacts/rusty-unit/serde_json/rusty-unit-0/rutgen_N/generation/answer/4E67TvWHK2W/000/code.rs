// Answer 0

#[test]
fn test_parse_decimal_positive_significand() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.index).copied()
        }

        fn peek(&self) -> Option<u8> {
            self.peek_or_null()
        }

        fn parse_decimal_overflow(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            // Placeholder for overflow behavior, assuming a certain result for the sake of testing
            Ok(if positive { f64::INFINITY } else { f64::NEG_INFINITY })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            let final_number = if positive { significand as f64 } else { -(significand as f64) };
            Ok(final_number * 10f64.powi(exponent))
        }
    }

    let mut parser = MockParser::new(b"123.45".to_vec());
    let result = parser.parse_decimal(true, 123, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 123.45);
}

#[test]
fn test_parse_decimal_negative_significand() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.index).copied()
        }

        fn peek(&self) -> Option<u8> {
            self.peek_or_null()
        }

        fn parse_decimal_overflow(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            Ok(if positive { f64::INFINITY } else { f64::NEG_INFINITY })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            let final_number = if positive { significand as f64 } else { -(significand as f64) };
            Ok(final_number * 10f64.powi(exponent))
        }
    }

    let mut parser = MockParser::new(b"-123.45".to_vec());
    let result = parser.parse_decimal(false, 123, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -123.45);
}

#[test]
fn test_parse_decimal_no_digits_after_decimal() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.index).copied()
        }

        fn peek(&self) -> Option<u8> {
            self.peek_or_null()
        }

        fn parse_decimal_overflow(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            Ok(if positive { f64::INFINITY } else { f64::NEG_INFINITY })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            Ok(if positive { significand as f64 } else { -(significand as f64) })
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }
    }

    let mut parser = MockParser::new(b"123.".to_vec());
    let result = parser.parse_decimal(true, 123, 0);
    assert!(result.is_err());
}

#[test]
fn test_parse_decimal_e_notation() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.index).copied()
        }

        fn peek(&self) -> Option<u8> {
            self.peek_or_null()
        }

        fn parse_decimal_overflow(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            Ok(if positive { f64::INFINITY } else { f64::NEG_INFINITY })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            Ok(if positive { significand as f64 } else { -(significand as f64) } * 10f64.powi(exponent))
        }

        fn parse_exponent(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64> {
            let exponent_result = exponent + 2; // Example modification for the exponent
            self.f64_from_parts(positive, significand, exponent_result)
        }
    }

    let mut parser = MockParser::new(b"1.23e2".to_vec());
    let result = parser.parse_decimal(true, 123, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 123.0);
}

