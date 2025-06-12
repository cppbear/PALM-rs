// Answer 0

#[test]
fn test_parse_exponent_positive() {
    struct TestParser {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            TestParser { input, cursor: 0 }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.input.get(self.cursor).cloned().ok_or(())
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.cursor < self.input.len() {
                let ch = self.input[self.cursor];
                self.eat_char();
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            // Simulate error handling
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            let base = if positive { significand as f64 } else { -(significand as f64) };
            Ok(base * 10f64.powi(exponent))
        }

        fn parse_exponent_overflow(&self, _: bool, _: bool, _: bool) -> Result<f64, ()> {
            // Simulate overflow handling
            Ok(f64::INFINITY)
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64, ()> {
            self.eat_char();

            let positive_exp = match self.peek_or_null() {
                Ok(b'+') => {
                    self.eat_char();
                    true
                }
                Ok(b'-') => {
                    self.eat_char();
                    false
                }
                _ => true,
            };

            let next = match self.next_char() {
                Ok(Some(b)) => b,
                Ok(None) => return Err(()), // Simulate EOF
                Err(_) => return Err(()), // Simulate error
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => return Err(()), // Simulate invalid number error
            };

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp.checked_mul(10).and_then(|e| e.checked_add(digit)).is_none() {
                        return self.parse_exponent_overflow(positive, significand == 0, positive_exp);
                    }
                    exp = exp * 10 + digit;
                } else {
                    break;
                }
            }

            let final_exp = if positive_exp {
                starting_exp.saturating_add(exp)
            } else {
                starting_exp.saturating_sub(exp)
            };

            self.f64_from_parts(positive, significand, final_exp)
        }
    }

    let mut parser = TestParser::new(vec![b'+', b'1', b'2', b'3', b'0']);
    let result = parser.parse_exponent(true, 123, 5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1.23e6);
}

#[test]
fn test_parse_exponent_negative() {
    struct TestParser {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            TestParser { input, cursor: 0 }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.input.get(self.cursor).cloned().ok_or(())
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.cursor < self.input.len() {
                let ch = self.input[self.cursor];
                self.eat_char();
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            // Simulate error handling
        }

        fn f64_from_parts(&self, positive: bool, significand: u64, exponent: i32) -> Result<f64, ()> {
            let base = if positive { significand as f64 } else { -(significand as f64) };
            Ok(base * 10f64.powi(exponent))
        }

        fn parse_exponent_overflow(&self, _: bool, _: bool, _: bool) -> Result<f64, ()> {
            // Simulate overflow handling
            Ok(f64::INFINITY)
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64, ()> {
            self.eat_char();

            let positive_exp = match self.peek_or_null() {
                Ok(b'+') => {
                    self.eat_char();
                    true
                }
                Ok(b'-') => {
                    self.eat_char();
                    false
                }
                _ => true,
            };

            let next = match self.next_char() {
                Ok(Some(b)) => b,
                Ok(None) => return Err(()), // Simulate EOF
                Err(_) => return Err(()), // Simulate error
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => return Err(()), // Simulate invalid number error
            };

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp.checked_mul(10).and_then(|e| e.checked_add(digit)).is_none() {
                        return self.parse_exponent_overflow(positive, significand == 0, positive_exp);
                    }
                    exp = exp * 10 + digit;
                } else {
                    break;
                }
            }

            let final_exp = if positive_exp {
                starting_exp.saturating_add(exp)
            } else {
                starting_exp.saturating_sub(exp)
            };

            self.f64_from_parts(positive, significand, final_exp)
        }
    }

    let mut parser = TestParser::new(vec![b'-', b'2', b'5', b'0']);
    let result = parser.parse_exponent(false, 250, 5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -2.5e6);
}

