// Answer 0

#[test]
fn test_parse_long_integer_success_case() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err("end of input")
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(0.0) // dummy implementation
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(0.0) // dummy implementation
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(0.0) // dummy implementation
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

    let mut parser = MockParser {
        input: b"12345".to_vec(),
        index: 0,
    };
    let result = parser.parse_long_integer(true, 12345);
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
#[should_panic(expected = "end of input")]
fn test_parse_long_integer_empty_input() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err("end of input")
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: usize) -> Result<f64, &'static str> {
            Ok(0.0) // dummy implementation
        }

        fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64, &'static str> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null()? {
                    _ => break,
                }
            }
        }
    }

    let mut parser = MockParser {
        input: Vec::new(),
        index: 0,
    };
    let _result = parser.parse_long_integer(true, 0);
}

