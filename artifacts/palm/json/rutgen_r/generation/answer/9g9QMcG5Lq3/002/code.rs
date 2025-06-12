// Answer 0

#[test]
fn test_parse_exponent_overflow_with_zero_significand() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> () {
            panic!("Error occurred");
        }

        fn parse_exponent_overflow(
            &mut self,
            positive: bool,
            zero_significand: bool,
            positive_exp: bool,
        ) -> Result<f64, ()> {
            if !zero_significand && positive_exp {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                } else {
                    break;
                }
            }
            Ok(if positive { 0.0 } else { -0.0 })
        }
    }

    let mut parser = MockParser { 
        input: vec![b'0'], // Simulating input with a zero character
        index: 0,
    };

    let result = parser.parse_exponent_overflow(true, true, false);
    assert_eq!(result, Ok(0.0));

    let result = parser.parse_exponent_overflow(false, true, false);
    assert_eq!(result, Ok(-0.0));
} 

#[test]
fn test_parse_exponent_overflow_with_error() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(()) // Triggering an error condition
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> () {
            panic!("Error occurred");
        }

        fn parse_exponent_overflow(
            &mut self,
            positive: bool,
            zero_significand: bool,
            positive_exp: bool,
        ) -> Result<f64, ()> {
            if !zero_significand && positive_exp {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                } else {
                    break;
                }
            }
            Ok(if positive { 0.0 } else { -0.0 })
        }
    }

    let mut parser = MockParser { 
        input: vec![], // Simulating empty input to trigger an error
        index: 0,
    };

    let result = parser.parse_exponent_overflow(true, true, false);
    assert_eq!(result, Ok(0.0));

    let result = parser.parse_exponent_overflow(false, true, false);
    assert_eq!(result, Ok(-0.0));

    let error_result = parser.parse_exponent_overflow(true, false, true);
    assert!(error_result.is_err());
}

