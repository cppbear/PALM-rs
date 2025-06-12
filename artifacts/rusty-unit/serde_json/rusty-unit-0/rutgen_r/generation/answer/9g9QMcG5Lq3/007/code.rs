// Answer 0

#[test]
fn test_parse_exponent_overflow_positive_exp_zero_significand_false() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, index: 0 }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            // Mock error implementation for testing
            Error::new("Number out of range")
        }

        fn parse_exponent_overflow(
            &mut self,
            positive: bool,
            zero_significand: bool,
            positive_exp: bool,
        ) -> Result<f64> {
            if !zero_significand && positive_exp {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }

            while let b'0'..=b'9' = self.peek_or_null()? {
                self.eat_char();
            }
            Ok(if positive { 0.0 } else { -0.0 })
        }
    }

    let mut parser = MockParser::new(vec![b'1', b'2', b'3']); // Non-zero significand, positive exponent
    let result = parser.parse_exponent_overflow(true, false, true);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.to_string(), "Number out of range");
}

