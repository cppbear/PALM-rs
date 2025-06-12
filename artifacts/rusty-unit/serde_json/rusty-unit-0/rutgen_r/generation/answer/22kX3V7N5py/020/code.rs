// Answer 0

#[test]
fn test_parse_any_signed_number_negative() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn parse_any_number(&mut self, _is_positive: bool) -> Result<ParserNumber, ()> {
            // Simulating a parsing function that fails for demonstration
            Err(())
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Handle peek error
        }

        fn fix_position(&self, _err: ()) -> () {
            // Fix position of the error
        }

        fn parse_any_signed_number(&mut self) -> Result<ParserNumber, ()> {
            let peek = match self.peek() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(self.peek_error(ErrorCode::InvalidNumber)),
            };

            match self.peek() {
                Ok(Some(_)) => Err(self.peek_error(ErrorCode::InvalidNumber)),
                _ => value,
            }
        }
    }

    let mut parser = TestParser {
        input: b"-123".to_vec(),
        position: 0,
    };

    let result = parser.parse_any_signed_number();
    assert_eq!(result, Err(()));
}

#[test]
fn test_parse_any_signed_number_invalid() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn parse_any_number(&mut self, _is_positive: bool) -> Result<ParserNumber, ()> {
            Err(())
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Handle peek error
        }

        fn fix_position(&self, _err: ()) -> () {
            // Fix position of the error
        }

        fn parse_any_signed_number(&mut self) -> Result<ParserNumber, ()> {
            let peek = match self.peek() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(self.peek_error(ErrorCode::InvalidNumber)),
            };

            match self.peek() {
                Ok(Some(_)) => Err(self.peek_error(ErrorCode::InvalidNumber)),
                _ => value,
            }
        }
    }

    let mut parser = TestParser {
        input: b"abc".to_vec(),
        position: 0,
    };

    let result = parser.parse_any_signed_number();
    assert_eq!(result, Err(()));
}

