// Answer 0

#[test]
fn test_parse_decimal_overflow_pos_exponent() {
    struct MockParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Ok(100.0) // Simulating a valid result from parsing exponent
        }

        fn f64_from_parts(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Ok(0.0) // Simulating a valid result from f64 conversion
        }

        fn parse_decimal_overflow(&mut self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            // The next multiply/add would overflow, so just ignore all further digits.
            while let Ok(val) = self.peek_or_null() {
                if val < b'0' || val > b'9' {
                    break;
                }
                self.eat_char();
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.parse_exponent(positive, significand, exponent),
                _ => self.f64_from_parts(positive, significand, exponent),
            }
        }
    }

    let mut parser = MockParser { data: b"12345e10".to_vec(), pos: 0 };
    let result = parser.parse_decimal_overflow(true, 12345, 10);
    assert_eq!(result, Ok(100.0));
}

#[test]
fn test_parse_decimal_overflow_no_exponent() {
    struct MockParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Ok(100.0) // Simulating a valid result from parsing exponent
        }

        fn f64_from_parts(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Ok(12345.0) // Simulating a valid result from f64 conversion
        }

        fn parse_decimal_overflow(&mut self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            while let Ok(val) = self.peek_or_null() {
                if val < b'0' || val > b'9' {
                    break;
                }
                self.eat_char();
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.parse_exponent(positive, significand, exponent),
                _ => self.f64_from_parts(positive, significand, exponent),
            }
        }
    }

    let mut parser = MockParser { data: b"67890".to_vec(), pos: 0 };
    let result = parser.parse_decimal_overflow(true, 67890, 0);
    assert_eq!(result, Ok(12345.0));
}

#[test]
#[should_panic]
fn test_parse_decimal_overflow_empty_input() {
    struct MockParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Ok(100.0) // Simulating a result
        }

        fn f64_from_parts(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Ok(0.0) // Simulating a result
        }

        fn parse_decimal_overflow(&mut self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            while let Ok(val) = self.peek_or_null() {
                if val < b'0' || val > b'9' {
                    break;
                }
                self.eat_char();
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.parse_exponent(positive, significand, exponent),
                _ => self.f64_from_parts(positive, significand, exponent),
            }
        }
    }

    let mut parser = MockParser { data: b"".to_vec(), pos: 0 };
    parser.parse_decimal_overflow(true, 0, 0);
}

