// Answer 0

#[test]
fn test_parse_long_integer_valid_case() {
    struct MockParser {
        index: usize,
        chars: Vec<u8>,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn parse_decimal(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            // Implement a simple mock for parsing decimal.
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn parse_exponent(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            // Mock implementation for exponent parsing
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64, &'static str> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null()? {
                    b'0'..=b'9' => {
                        self.eat_char();
                        exponent += 1;
                    }
                    b'.' => {
                        return self.parse_decimal(positive, significand, exponent);
                    }
                    b'e' | b'E' => {
                        return self.parse_exponent(positive, significand, exponent);
                    }
                    _ => {
                        return self.f64_from_parts(positive, significand, exponent);
                    }
                }
            }
        }
    }

    let mut parser = MockParser { index: 0, chars: b"1234e5".to_vec() };
    let result = parser.parse_long_integer(true, 1234);
    assert_eq!(result.unwrap(), 123400.0);
}

#[test]
fn test_parse_long_integer_exponent_case() {
    struct MockParser {
        index: usize,
        chars: Vec<u8>,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn parse_decimal(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn parse_exponent(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64, &'static str> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null()? {
                    b'0'..=b'9' => {
                        self.eat_char();
                        exponent += 1;
                    }
                    b'.' => {
                        return self.parse_decimal(positive, significand, exponent);
                    }
                    b'e' | b'E' => {
                        return self.parse_exponent(positive, significand, exponent);
                    }
                    _ => {
                        return self.f64_from_parts(positive, significand, exponent);
                    }
                }
            }
        }
    }

    let mut parser = MockParser { index: 0, chars: b"5678E2".to_vec() };
    let result = parser.parse_long_integer(true, 5678);
    assert_eq!(result.unwrap(), 567800.0);
}

#[test]
fn test_parse_long_integer_invalid_end() {
    struct MockParser {
        index: usize,
        chars: Vec<u8>,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn parse_decimal(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn parse_exponent(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10_f64.powi(exponent) } else { -(significand as f64 * 10_f64.powi(exponent)) })
        }

        fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64, &'static str> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null()? {
                    b'0'..=b'9' => {
                        self.eat_char();
                        exponent += 1;
                    }
                    b'.' => {
                        return self.parse_decimal(positive, significand, exponent);
                    }
                    b'e' | b'E' => {
                        return self.parse_exponent(positive, significand, exponent);
                    }
                    _ => {
                        return self.f64_from_parts(positive, significand, exponent);
                    }
                }
            }
        }
    }

    let mut parser = MockParser { index: 0, chars: b"1234$".to_vec() };
    let result = parser.parse_long_integer(true, 1234);
    assert!(result.is_ok());
}

