// Answer 0

fn parse_exponent_test() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err("End of input")
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.eat_char();
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "Error occurred"
        }
        
        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, &'static str> {
            // stub
            Ok(0.0)
        }

        fn parse_exponent_overflow(&self, positive: bool, zero_significand: bool, _positive_exp: bool) -> Result<f64, &'static str> {
            // stub for overflow case
            Ok(0.0)
        }
            
        fn parse_exponent(&mut self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64, &'static str> {
            self.eat_char();

            let positive_exp = match self.peek_or_null() {
                Ok(b'+') => {
                    self.eat_char();
                    true
                }
                Ok(b'-') => {
                    self.eat_char();
                    false
                }
                _ => true,
            };

            let next = match self.next_char() {
                Ok(Some(b)) => b,
                _ => return Err(self.error(ErrorCode::EofWhileParsingValue)),
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => return Err(self.error(ErrorCode::InvalidNumber)),
            };

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                    let digit = (c - b'0') as i32;
                    if overflow!(exp * 10 + digit, i32::MAX) {
                        let zero_significand = significand == 0;
                        return self.parse_exponent_overflow(positive, zero_significand, positive_exp);
                    }
                    exp = exp * 10 + digit;
                } else {
                    break;
                }
            }

            let final_exp = if positive_exp {
                starting_exp.saturating_add(exp)
            } else {
                starting_exp.saturating_sub(exp)
            };

            self.f64_from_parts(positive, significand, final_exp)
        }
    }

    #[test]
    fn test_parse_exponent_error_eof() {
        let mut parser = MockParser::new(vec![b'e', b'+' /* valid exponent indicator, but no digits after */]);
        let result = parser.parse_exponent(true, 123, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_exponent_invalid_number() {
        let mut parser = MockParser::new(vec![b'e', b'-', b'a' /* 'a' is invalid as exponent digit */]);
        let result = parser.parse_exponent(true, 123, 10);
        assert!(result.is_err());
    }
}

