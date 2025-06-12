// Answer 0

#[test]
fn test_parse_exponent_valid_case_positive() {
    struct TestParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                let c = self.chars[self.index];
                self.eat_char();
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            let value = if positive {
                significand as f64 * 10f64.powi(exponent)
            } else {
                -(significand as f64 * 10f64.powi(exponent))
            };
            Ok(value)
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(())
        }

        fn error(&self, _: ()) -> () {
            ()
        }
    }

    let mut parser = TestParser {
        chars: b"x+123".to_vec(),
        index: 0,
    };

    let result = parser.parse_exponent(true, 1, 0);
    assert!(result.is_ok());
}

#[test]
fn test_parse_exponent_valid_case_negative() {
    struct TestParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                let c = self.chars[self.index];
                self.eat_char();
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            let value = if positive {
                significand as f64 * 10f64.powi(exponent)
            } else {
                -(significand as f64 * 10f64.powi(exponent))
            };
            Ok(value)
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(())
        }

        fn error(&self, _: ()) -> () {
            ()
        }
    }

    let mut parser = TestParser {
        chars: b"x-456".to_vec(),
        index: 0,
    };

    let result = parser.parse_exponent(false, 1, 0);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_exponent_invalid_character() {
    struct TestParser {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                let c = self.chars[self.index];
                self.eat_char();
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
            ()
        }
    }

    let mut parser = TestParser {
        chars: b"x&123".to_vec(),
        index: 0,
    };

    let _ = parser.parse_exponent(true, 1, 0);
}

