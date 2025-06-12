// Answer 0

#[test]
fn test_parse_long_integer_no_decimal() {
    struct MockParser {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Err("End of input")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.chars.len() {
                self.position += 1;
            }
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, &'static str> {
            // For this test, return an arbitrary value as we are not testing this path.
            Ok(0.0)
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, &'static str> {
            // For this test, return an arbitrary value as we are not testing this path.
            Ok(0.0)
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, &'static str> {
            // Actual conversion logic would go here.
            Ok(1.0) // Return a dummy value for the sake of this test.
        }

        fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64, &'static str> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null() {
                    Ok(val) if (b'0'..=b'9').contains(&val) => {
                        self.eat_char();
                        exponent += 1;
                    }
                    Ok(b'.') => {
                        return self.parse_decimal(positive, significand, exponent);
                    }
                    Ok(b'e') | Ok(b'E') => {
                        return self.parse_exponent(positive, significand, exponent);
                    }
                    _ => {
                        return self.f64_from_parts(positive, significand, exponent);
                    }
                }
            }
        }
    }

    let mut parser = MockParser {
        chars: vec![b'1', b'2', b'3'], // Valid digits
        position: 0,
    };

    let result = parser.parse_long_integer(true, 123);
    assert_eq!(result.unwrap(), 1.0); // Expecting the dummy return value from f64_from_parts
}

#[test]
fn test_parse_long_integer_invalid_character() {
    struct MockParser {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Err("End of input")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.chars.len() {
                self.position += 1;
            }
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, &'static str> {
            Ok(0.0)
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, &'static str> {
            Ok(0.0)
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, &'static str> {
            Ok(1.0)
        }

        fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64, &'static str> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null() {
                    Ok(val) if (b'0'..=b'9').contains(&val) => {
                        self.eat_char();
                        exponent += 1;
                    }
                    Ok(b'.') => {
                        return self.parse_decimal(positive, significand, exponent);
                    }
                    Ok(b'e') | Ok(b'E') => {
                        return self.parse_exponent(positive, significand, exponent);
                    }
                    _ => {
                        return self.f64_from_parts(positive, significand, exponent);
                    }
                }
            }
        }
    }

    let mut parser = MockParser {
        chars: vec![b'1', b'2', b'3', b'x'], // Invalid character
        position: 0,
    };

    let result = parser.parse_long_integer(true, 123);
    assert_eq!(result.unwrap(), 1.0); // Expecting the dummy return value from f64_from_parts
}

