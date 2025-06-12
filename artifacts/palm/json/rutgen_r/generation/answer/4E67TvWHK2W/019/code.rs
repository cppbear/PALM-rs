// Answer 0

fn test_parse_decimal_valid_input() {
    struct MockParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.bytes().collect(),
                pos: 0,
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0) // Simulating a null value
            }
        }

        fn peek(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err(())
            }
        }

        fn parse_decimal_overflow(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            // Simulating overflow handling
            Ok(if positive { f64::INFINITY } else { f64::NEG_INFINITY })
        }

        fn parse_exponent(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            // Simulating exponent parsing
            Ok(significand as f64 * 10f64.powi(exponent))
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            // Converting to float
            Ok(if positive {
                (significand as f64) * 10f64.powi(exponent)
            } else {
                -(significand as f64) * 10f64.powi(exponent)
            })
        }
    }

    let mut parser = MockParser::new("123.456e2");
    let result = parse_decimal(&mut parser, true, 123, 0);
    assert_eq!(result, Ok(12345600.0)); // Expecting result after parsing
}

fn test_parse_decimal_no_digits_after_decimal() {
    struct MockParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.bytes().collect(),
                pos: 0,
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn peek(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err(())
            }
        }

        fn parse_decimal_overflow(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            Ok(if positive { f64::INFINITY } else { f64::NEG_INFINITY })
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            Ok(if positive {
                (significand as f64) * 10f64.powi(exponent)
            } else {
                -(significand as f64) * 10f64.powi(exponent)
            })
        }
    }

    let mut parser = MockParser::new("123.");
    let result = parse_decimal(&mut parser, true, 123, 0);
    assert!(result.is_err()); // Expecting an error due to no digits after decimal
}

fn test_parse_decimal_exponent_present() {
    struct MockParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.bytes().collect(),
                pos: 0,
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn peek(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err(())
            }
        }

        fn parse_decimal_overflow(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            Ok(if positive { f64::INFINITY } else { f64::NEG_INFINITY })
        }

        fn parse_exponent(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            Ok(significand as f64 * 10f64.powi(exponent))
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            Ok(if positive {
                (significand as f64) * 10f64.powi(exponent)
            } else {
                -(significand as f64) * 10f64.powi(exponent)
            })
        }
    }

    let mut parser = MockParser::new("123.456e-2");
    let result = parse_decimal(&mut parser, true, 123, 0);
    assert_eq!(result, Ok(1.23456)); // Expecting result after parsing the exponent
}

