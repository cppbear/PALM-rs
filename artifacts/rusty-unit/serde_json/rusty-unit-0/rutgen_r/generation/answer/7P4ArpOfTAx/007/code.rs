// Answer 0

#[test]
fn test_parse_decimal_overflow_empty_input() {
    struct DummyParser {
        input: Vec<u8>,
        index: usize,
    }

    impl DummyParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err("EOF")
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Err("Exponent parsing not implemented")
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Err("Conversion not implemented")
        }
    }

    let mut parser = DummyParser { input: vec![], index: 0 };
    let result = parser.parse_decimal_overflow(true, 0, 0);
    assert_eq!(result, Err("EOF"));
}

#[test]
fn test_parse_decimal_overflow_invalid_character() {
    struct DummyParser {
        input: Vec<u8>,
        index: usize,
    }

    impl DummyParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err("EOF")
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Err("Exponent parsing not implemented")
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Err("Conversion not implemented")
        }
    }

    let mut parser = DummyParser { input: vec![b'a'], index: 0 };
    let result = parser.parse_decimal_overflow(true, 0, 0);
    assert_eq!(result, Err("EOF"));
}

#[test]
fn test_parse_decimal_overflow_exponent_followed_by_invalid_character() {
    struct DummyParser {
        input: Vec<u8>,
        index: usize,
    }

    impl DummyParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err("EOF")
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Err("Exponent parsing not implemented")
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Err("Conversion not implemented")
        }
    }

    let mut parser = DummyParser { input: vec![b'1', b'e', b'a'], index: 0 };
    let result = parser.parse_decimal_overflow(true, 1, 0);
    assert_eq!(result, Err("EOF"));
}

#[test]
fn test_parse_decimal_overflow_valid_input_without_exponent() {
    struct DummyParser {
        input: Vec<u8>,
        index: usize,
    }

    impl DummyParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err("EOF")
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &'static str> {
            Err("Exponent parsing not implemented")
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &'static str> {
            if positive {
                Ok(significand as f64 * 10f64.powi(exponent))
            } else {
                Err("Negative values not supported")
            }
        }
    }

    let mut parser = DummyParser { input: vec![b'3', b'4', b'5'], index: 0 };
    let result = parser.parse_decimal_overflow(true, 345, 0);
    assert_eq!(result, Ok(345.0));
}

