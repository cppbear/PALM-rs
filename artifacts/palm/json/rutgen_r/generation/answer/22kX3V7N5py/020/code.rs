// Answer 0

#[test]
fn test_parse_any_signed_number_eof_error() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.index]))
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_any_number(&self, _positive: bool) -> Result<u32, &'static str> {
            Ok(0) // Simplified for testing
        }

        fn peek_error(&self, _code: &'static str) -> &'static str {
            "Unexpected end of input"
        }
        
        fn fix_position(&self, err: &'static str) -> &'static str {
            err // Simplified for testing
        }

        fn parse_any_signed_number(&mut self) -> Result<u32, &'static str> {
            let peek = match self.peek()? {
                Some(b) => b,
                None => return Err(self.peek_error("EofWhileParsingValue")),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(self.peek_error("InvalidNumber")),
            };

            match self.peek() {
                Ok(Some(_)) => Err(self.peek_error("InvalidNumber")),
                Ok(None) => value,
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = TestParser {
        input: vec![], // Empty input to trigger EOF
        index: 0,
    };

    let result = parser.parse_any_signed_number();
    assert_eq!(result, Err("Unexpected end of input"));
}

#[test]
fn test_parse_any_signed_number_invalid_number() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.index >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.index]))
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_any_number(&self, _positive: bool) -> Result<u32, &'static str> {
            Err("Invalid number") // Triggering invalid number error
        }

        fn peek_error(&self, _code: &'static str) -> &'static str {
            "InvalidNumber"
        }
        
        fn fix_position(&self, err: &'static str) -> &'static str {
            err // Simplified for testing
        }

        fn parse_any_signed_number(&mut self) -> Result<u32, &'static str> {
            let peek = match self.peek()? {
                Some(b) => b,
                None => return Err(self.peek_error("EofWhileParsingValue")),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(self.peek_error("InvalidNumber")),
            };

            match self.peek() {
                Ok(Some(_)) => Err(self.peek_error("InvalidNumber")),
                Ok(None) => value,
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = TestParser {
        input: vec![b'-', b'1'], // Input leading with '-'
        index: 0,
    };

    let result = parser.parse_any_signed_number();
    assert_eq!(result, Err("InvalidNumber"));
}

