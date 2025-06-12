// Answer 0

#[test]
fn test_parse_decimal_overflow_valid_case() {
    struct TestParser {
        data: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
                position: 0,
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err("End of data")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            // Simulating a proper exponent parse scenario, omitted for brevity
            Ok(if positive { significand as f64 * 10f64.powi(exponent) } else { -significand as f64 * 10f64.powi(exponent) })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10f64.powi(exponent) } else { -significand as f64 * 10f64.powi(exponent) })
        }

        fn parse_decimal_overflow(
            &mut self,
            positive: bool,
            significand: u64,
            exponent: i32,
        ) -> Result<f64, &'static str> {
            while let Ok(val) = self.peek_or_null() {
                if val >= b'0' && val <= b'9' {
                    self.eat_char();
                } else {
                    break;
                }
            }
            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.parse_exponent(positive, significand, exponent),
                Ok(_) => self.f64_from_parts(positive, significand, exponent),
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = TestParser::new(b"12345E2");
    let result = parser.parse_decimal_overflow(true, 12345, 2);
    assert_eq!(result, Ok(1234500.0));
}

#[test]
fn test_parse_decimal_overflow_invalid_case() {
    struct TestParser {
        data: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
                position: 0,
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err("End of data")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10f64.powi(exponent) } else { -significand as f64 * 10f64.powi(exponent) })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10f64.powi(exponent) } else { -significand as f64 * 10f64.powi(exponent) })
        }

        fn parse_decimal_overflow(
            &mut self,
            positive: bool,
            significand: u64,
            exponent: i32,
        ) -> Result<f64, &'static str> {
            while let Ok(val) = self.peek_or_null() {
                if val >= b'0' && val <= b'9' {
                    self.eat_char();
                } else {
                    break;
                }
            }
            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.parse_exponent(positive, significand, exponent),
                Ok(_) => self.f64_from_parts(positive, significand, exponent),
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = TestParser::new(b"12345F");
    let result = parser.parse_decimal_overflow(true, 12345, 2);
    assert_eq!(result, Ok(1234500.0));
}

#[test]
#[should_panic]
fn test_parse_decimal_overflow_panic_case() {
    struct TestParser {
        data: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
                position: 0,
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err("End of data")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10f64.powi(exponent) } else { -significand as f64 * 10f64.powi(exponent) })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            Ok(if positive { significand as f64 * 10f64.powi(exponent) } else { -significand as f64 * 10f64.powi(exponent) })
        }

        fn parse_decimal_overflow(
            &mut self,
            positive: bool,
            significand: u64,
            exponent: i32,
        ) -> Result<f64, &'static str> {
            while let Ok(val) = self.peek_or_null() {
                if val >= b'0' && val <= b'9' {
                    self.eat_char();
                } else {
                    break;
                }
            }
            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.parse_exponent(positive, significand, exponent),
                Ok(_) => self.f64_from_parts(positive, significand, exponent),
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = TestParser::new(b"");
    let _ = parser.parse_decimal_overflow(true, 12345, 2);
}

