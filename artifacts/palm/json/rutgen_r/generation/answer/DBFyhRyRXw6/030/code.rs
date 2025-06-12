// Answer 0

#[test]
fn test_parse_exponent_positive() {
    struct TestParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.pos < self.data.len() {
                let c = self.data[self.pos];
                self.pos += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> String {
            "Error".to_string()
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(0.0) // Placeholder implementation
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(()) // Placeholder implementation for testing purposes
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
                _ => return Err(self.error(())),
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => return Err(self.error(())),
            };

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp.checked_mul(10).is_none() || exp.checked_add(digit).is_none() {
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

    let mut parser = TestParser { data: b"e+12".to_vec(), pos: 0 };
    let result = parser.parse_exponent(true, 1, 0);
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn test_parse_exponent_negative() {
    struct TestParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.pos < self.data.len() {
                let c = self.data[self.pos];
                self.pos += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> String {
            "Error".to_string()
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(0.0) // Placeholder implementation
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(()) // Placeholder implementation for testing purposes
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
                _ => return Err(self.error(())),
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => return Err(self.error(())),
            };

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp.checked_mul(10).is_none() || exp.checked_add(digit).is_none() {
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

    let mut parser = TestParser { data: b"e-5".to_vec(), pos: 0 };
    let result = parser.parse_exponent(true, 1, 20);
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn test_parse_exponent_invalid_char() {
    struct TestParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.pos < self.data.len() {
                let c = self.data[self.pos];
                self.pos += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> String {
            "Error".to_string()
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(0.0) // Placeholder implementation
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(()) // Placeholder implementation for testing purposes
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
                _ => return Err(self.error(())),
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => return Err(self.error(())),
            };

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp.checked_mul(10).is_none() || exp.checked_add(digit).is_none() {
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

    let mut parser = TestParser { data: b"e*".to_vec(), pos: 0 };
    let result = parser.parse_exponent(true, 1, 0);
    assert!(result.is_err());
}

#[test]
fn test_parse_exponent_overflow() {
    struct TestParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.data.len() {
                Ok(self.data[self.pos])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.pos < self.data.len() {
                let c = self.data[self.pos];
                self.pos += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> String {
            "Error".to_string()
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(0.0) // Placeholder implementation
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(()) // This should trigger overflow
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
                _ => return Err(self.error(())),
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => return Err(self.error(())),
            };

            while let Ok(c) = self.peek_or_null() {
                if c >= b'0' && c <= b'9' {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp.checked_mul(10).is_none() || exp.checked_add(digit).is_none() {
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

    let mut parser = TestParser { data: b"e1234567890".to_vec(), pos: 0 };
    let result = parser.parse_exponent(true, 1, i32::MAX);
    assert!(result.is_err());
}

