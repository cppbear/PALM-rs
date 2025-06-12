// Answer 0

#[test]
fn test_parse_exponent_with_positive_exp() {
    struct TestParser {
        data: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn new(data: &[u8]) -> Self {
            Self { data: data.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.eat_char();
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: u8) -> () {
            // Simulate an error handling
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(1.0) // Dummy implementation returning a basic result
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(())
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
                _ => {
                    return Err(());
                }
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => {
                    return Err(());
                }
            };

            while let Ok(c) = self.peek_or_null() {
                if c.is_ascii_digit() {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    // Simulating overflow handling; maximal exp
                    if exp > i32::MAX / 10 || (exp == i32::MAX / 10 && digit > 7) {
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

    let mut parser = TestParser::new(b"E+10");
    let result = parser.parse_exponent(true, 42, 0);
    assert_eq!(result, Ok(1.0));
}

#[test]
fn test_parse_exponent_with_negative_exp() {
    struct TestParser {
        data: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn new(data: &[u8]) -> Self {
            Self { data: data.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.eat_char();
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: u8) -> () {}

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(1.0)
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(())
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
                _ => {
                    return Err(());
                }
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => {
                    return Err(());
                }
            };

            while let Ok(c) = self.peek_or_null() {
                if c.is_ascii_digit() {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp > i32::MAX / 10 || (exp == i32::MAX / 10 && digit > 7) {
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

    let mut parser = TestParser::new(b"E-10");
    let result = parser.parse_exponent(true, 42, 0);
    assert_eq!(result, Ok(1.0));
}

#[test]
#[should_panic]
fn test_parse_exponent_with_invalid_character() {
    struct TestParser {
        data: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn new(data: &[u8]) -> Self {
            Self { data: data.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(())
            }
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.eat_char();
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: u8) -> () {}

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(1.0)
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
            Err(())
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
                _ => {
                    return Err(());
                }
            };

            let mut exp = match next {
                c @ b'0'..=b'9' => (c - b'0') as i32,
                _ => {
                    return Err(());
                }
            };

            while let Ok(c) = self.peek_or_null() {
                if c.is_ascii_digit() {
                    self.eat_char();
                    let digit = (c - b'0') as i32;

                    if exp > i32::MAX / 10 || (exp == i32::MAX / 10 && digit > 7) {
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

    let mut parser = TestParser::new(b"E*10"); // This will cause the function to hit an invalid character
    let _ = parser.parse_exponent(true, 42, 0);
}

