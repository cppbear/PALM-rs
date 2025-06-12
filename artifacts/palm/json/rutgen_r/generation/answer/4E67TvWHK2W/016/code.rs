// Answer 0

fn test_parse_decimal_empty_after_decimal() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &[u8]) -> Self {
            MockParser { input: input.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF").into())
            }
        }

        fn peek(&mut self) -> Result<u8> {
            self.peek_or_null()
        }

        fn parse_decimal_overflow(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Overflow").into())
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            Ok(0.0) // placeholder
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            Ok(0.0) // placeholder
        }
    }

    let mut parser = MockParser::new(b"1.0");
    parser.eat_char(); // consume '1'

    // This will cause exponent_after_decimal_point to be 0 with no digits after the decimal.
    let result = parser.parse_decimal(false, 1, 1);

    assert!(result.is_err());
}

fn test_parse_decimal_non_numeric_after_decimal() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &[u8]) -> Self {
            MockParser { input: input.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF").into())
            }
        }

        fn peek(&mut self) -> Result<u8> {
            self.peek_or_null()
        }

        fn parse_decimal_overflow(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Overflow").into())
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            Ok(0.0) // placeholder
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            Ok(0.0) // placeholder
        }
    }

    let mut parser = MockParser::new(b"0.abc");
    parser.eat_char(); // consume '0'
    parser.eat_char(); // consume '.'

    // This will cause an error with non-numeric data after the decimal point.
    let result = parser.parse_decimal(true, 0, 0);

    assert!(result.is_err());
}

