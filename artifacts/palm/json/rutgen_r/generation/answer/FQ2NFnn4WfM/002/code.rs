// Answer 0

#[test]
fn test_parse_long_integer_success() {
    struct MockParser {
        data: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err("end of data")
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(0.0) // Simplified return for the example
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(0.0) // Simplified return for the example
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(12345.0) // Simplified return for the example
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

    let mut parser = MockParser::new(vec![b'1', b'2', b'3', b'.']);
    let result = parser.parse_long_integer(true, 123);
    assert_eq!(result, Ok(0.0)); // Expected to call parse_decimal and return 0.0
}

#[test]
fn test_parse_long_integer_error() {
    struct MockParser {
        data: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err("end of data")
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(0.0) // Simplified return for the example
        }

        fn parse_long_integer(&mut self, _positive: bool, _significand: u64) -> Result<f64, &'static str> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null().map_err(|e| e)? {
                    b'0'..=b'9' => {
                        self.eat_char();
                        exponent += 1;
                    }
                    b'.' => {
                        return Ok(0.0); // Simplified return
                    }
                    b'e' | b'E' => {
                        return Ok(0.0); // Simplified return
                    }
                    _ => {
                        return self.f64_from_parts(false, 0, exponent);
                    }
                }
            }
        }
    }

    let mut parser = MockParser::new(vec![]);
    let result = parser.parse_long_integer(true, 0);
    assert_eq!(result, Err("end of data")); // Testing an empty input leading to error
}

