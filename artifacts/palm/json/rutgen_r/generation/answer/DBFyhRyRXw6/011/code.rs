// Answer 0

#[test]
fn test_parse_exponent_valid_positive_exp() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exp: i32) -> Result<f64, ()> {
            let value = if positive {
                significand as f64 * 10_f64.powi(exp)
            } else {
                -(significand as f64 * 10_f64.powi(exp))
            };
            Ok(value)
        }

        fn parse_exponent_overflow(&self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64, ()> {
            if zero_significand {
                Ok(0.0)
            } else {
                Err(())
            }
        }

        fn error(&self, _: ()) -> () {
            // Dummy error handling
        }
    }

    let mut parser = TestParser {
        input: b"e+12".to_vec(),
        index: 0,
    };
    
    let result = parser.parse_exponent(true, 123, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 123000000000.0);
}

#[test]
fn test_parse_exponent_valid_negative_exp() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exp: i32) -> Result<f64, ()> {
            let value = if positive {
                significand as f64 * 10_f64.powi(exp)
            } else {
                -(significand as f64 * 10_f64.powi(exp))
            };
            Ok(value)
        }

        fn parse_exponent_overflow(&self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64, ()> {
            if zero_significand {
                Ok(0.0)
            } else {
                Err(())
            }
        }

        fn error(&self, _: ()) -> () {
            // Dummy error handling
        }
    }

    let mut parser = TestParser {
        input: b"e-3".to_vec(),
        index: 0,
    };

    let result = parser.parse_exponent(false, 456, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -0.456);
}

#[test]
fn test_parse_exponent_invalid_character() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, _: bool, _: u64, _: i32) -> Result<f64, ()> {
            Err(())
        }

        fn parse_exponent_overflow(&self, _: bool, _: bool, _: bool) -> Result<f64, ()> {
            Err(())
        }

        fn error(&self, _: ()) -> () {
            // Dummy error handling
        }
    }

    let mut parser = TestParser {
        input: b"e$2".to_vec(),
        index: 0,
    };

    let result = parser.parse_exponent(true, 789, 0);
    assert!(result.is_err());
}

#[test]
fn test_parse_exponent_overflow() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, _: bool, _: u64, _: i32) -> Result<f64, ()> {
            Err(())
        }

        fn parse_exponent_overflow(&self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64, ()> {
            if zero_significand {
                Ok(0.0)
            } else {
                Err(())
            }
        }

        fn error(&self, _: ()) -> () {
            // Dummy error handling
        }
    }

    let mut parser = TestParser {
        input: b"e+10000000000".to_vec(),
        index: 0,
    };

    let result = parser.parse_exponent(true, 1, i32::MAX);
    assert!(result.is_err());
}

