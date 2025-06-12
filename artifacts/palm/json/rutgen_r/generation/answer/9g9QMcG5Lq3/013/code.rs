// Answer 0

#[test]
fn test_parse_exponent_overflow_negative_zero() {
    struct MockParser {
        peek_char: u8,
        index: usize,
    }
    
    impl MockParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, &str> {
            if self.index < 1 {
                Ok(self.peek_char)
            } else {
                Ok(b'\0') // Simulate end of digits
            }
        }

        fn error(&self, _code: ErrorCode) -> &str {
            "Number out of range"
        }
    }

    impl MockParser {
        fn parse_exponent_overflow(
            &mut self,
            positive: bool,
            zero_significand: bool,
            positive_exp: bool,
        ) -> Result<f64, &str> {
            if !zero_significand && positive_exp {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }
    
            while let b'0'..=b'9' = self.peek_or_null().unwrap() {
                self.eat_char();
            }
            Ok(if positive { 0.0 } else { -0.0 })
        }
    }

    let mut parser = MockParser { peek_char: b'1', index: 0 };
    let result = parser.parse_exponent_overflow(false, false, false);
    assert_eq!(result, Ok(-0.0));
}

