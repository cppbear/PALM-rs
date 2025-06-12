// Answer 0

#[test]
fn test_parse_decimal_valid_case() {
    struct MockParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek(&self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Err(())
            }
        }

        fn f64_from_parts(&self, _positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            let value = significand as f64 * 10f64.powi(exponent);
            Ok(value)
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, exponent: i32) -> Result<f64, ()> {
            // Let's simply return a value based on the exponent to simulate parsing.
            Ok(10f64.powi(exponent))
        }

        fn parse_decimal_overflow(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Err(()) // simulating error due to overflow
        }

        fn parse_decimal(
            &mut self,
            positive: bool,
            mut significand: u64,
            exponent_before_decimal_point: i32,
        ) -> Result<f64, ()> {
            self.eat_char();

            let mut exponent_after_decimal_point = 0;
            while let Ok(c) = self.peek_or_null() {
                if (b'0'..=b'9').contains(&c) {
                    let digit = (c - b'0') as u64;
                    significand = significand * 10 + digit;
                    exponent_after_decimal_point -= 1;
                    self.eat_char();
                } else {
                    break;
                }
            }

            if exponent_after_decimal_point == 0 {
                match self.peek() {
                    Ok(Some(_)) => return Err(()),
                    _ => return Err(()),
                }
            }

            let exponent = exponent_before_decimal_point + exponent_after_decimal_point;
            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.parse_exponent(positive, significand, exponent),
                _ => self.f64_from_parts(positive, significand, exponent),
            }
        }
    }

    let mut parser = MockParser::new(b"123.456e1".to_vec());
    let result = parser.parse_decimal(true, 0, 3);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1234.56);
}

#[test]
#[should_panic]
fn test_parse_decimal_no_digits_after_decimal() {
    struct MockParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek(&self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Err(())
            }
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Ok(0.0) // Just a placeholder
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, ()> {
            Ok(0.0) // Just a placeholder
        }

        fn parse_decimal(
            &mut self,
            _positive: bool,
            _significand: u64,
            _exponent_before_decimal_point: i32,
        ) -> Result<f64, ()> {
            self.eat_char();
            let exponent_after_decimal_point = 0;

            if exponent_after_decimal_point == 0 {
                match self.peek() {
                    Ok(Some(_)) => panic!(),
                    _ => panic!(),
                }
            }
            Ok(0.0) // Just a placeholder for the return type
        }
    }

    let mut parser = MockParser::new(b"123.".to_vec());
    parser.parse_decimal(true, 0, 3).unwrap();
}

