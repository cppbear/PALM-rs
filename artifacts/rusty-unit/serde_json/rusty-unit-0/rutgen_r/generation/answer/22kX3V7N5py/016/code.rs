// Answer 0

#[test]
fn test_parse_signed_number_negative() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, is_positive: bool) -> Result<ParserNumber, &'static str> {
            if self.index < self.input.len() {
                let num = if is_positive { self.input[self.index] } else { self.input[self.index] + 1 };
                self.index += 1;
                Ok(ParserNumber(num as i64))
            } else {
                Err("Out of bounds")
            }
        }
        
        fn fix_position(&self, err: &'static str) -> &'static str {
            err
        }
        
        fn peek_error(&self, code: ErrorCode) -> &'static str {
            "Peek error"
        }
        
        fn peek_error_result(&self, code: ErrorCode) -> Result<(), &'static str> {
            Err(self.peek_error(code))
        }
    }

    impl Iterator for MockParser {
        type Item = Result<ParserNumber, &'static str>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.input.len() {
                let peek = self.peek().ok().unwrap();
                match peek {
                    Some(b'-') => {
                        self.eat_char();
                        self.parse_any_number(false).ok()
                    }
                    Some(b'0'..=b'9') => {
                        self.parse_any_number(true).ok()
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
    }

    let mut parser = MockParser { input: b"-5".to_vec(), index: 0 };
    let result = parser.parse_any_signed_number().unwrap();
    assert_eq!(result, ParserNumber(-5));
}

#[test]
fn test_parse_signed_number_zero() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, is_positive: bool) -> Result<ParserNumber, &'static str> {
            if self.index < self.input.len() {
                let num = if is_positive { self.input[self.index] } else { self.input[self.index] + 1 };
                self.index += 1;
                Ok(ParserNumber(num as i64))
            } else {
                Err("Out of bounds")
            }
        }
        
        fn fix_position(&self, err: &'static str) -> &'static str {
            err
        }
        
        fn peek_error(&self, code: ErrorCode) -> &'static str {
            "Peek error"
        }
        
        fn peek_error_result(&self, code: ErrorCode) -> Result<(), &'static str> {
            Err(self.peek_error(code))
        }
    }

    let mut parser = MockParser { input: b"0".to_vec(), index: 0 };
    let result = parser.parse_any_signed_number().unwrap();
    assert_eq!(result, ParserNumber(0));
}

#[test]
fn test_parse_signed_number_invalid() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, _: bool) -> Result<ParserNumber, &'static str> {
            Err("Invalid number")
        }
        
        fn fix_position(&self, err: &'static str) -> &'static str {
            err
        }
        
        fn peek_error(&self, code: ErrorCode) -> &'static str {
            "Peek error"
        }
        
        fn peek_error_result(&self, code: ErrorCode) -> Result<(), &'static str> {
            Err(self.peek_error(code))
        }
    }

    let mut parser = MockParser { input: b"-".to_vec(), index: 0 };
    let result = parser.parse_any_signed_number();
    assert!(result.is_err());
}

