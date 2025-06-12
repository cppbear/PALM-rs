// Answer 0

fn test_parse_decimal_no_digits_after_decimal() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek(&self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn parse_decimal_overflow(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Err(())
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Ok(0.0) // Just for compilation, not relevant for this test
        }
        
        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Err(()) // Simulate that we won’t reach here
        }
    }

    let mut parser = MockParser {
        input: vec![b'1', b'.'], // input with no digits after decimal point
        index: 0,
    };

    let result = parser.parse_decimal(true, 1, 0);
    assert!(result.is_err());
}

fn test_parse_decimal_invalid_character_after_decimal() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek(&self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn parse_decimal_overflow(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Err(())
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Ok(0.0) // Just for compilation, not relevant for this test
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Err(()) // Simulate that we won’t reach here
        }
    }

    let mut parser = MockParser {
        input: vec![b'1', b'.', b'a'], // input with an invalid character after the decimal point
        index: 0,
    };

    let result = parser.parse_decimal(true, 1, 0);
    assert!(result.is_err());
}

fn test_parse_decimal_end_of_input_after_decimal() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek(&self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn parse_decimal_overflow(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Err(())
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Ok(0.0) // Just for compilation, not relevant for this test
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Err(()) // Simulate that we won’t reach here
        }
    }

    let mut parser = MockParser {
        input: vec![b'1', b'.'], // input with no characters afterwards
        index: 0,
    };

    let result = parser.parse_decimal(true, 1, 0);
    assert!(result.is_err());
}

