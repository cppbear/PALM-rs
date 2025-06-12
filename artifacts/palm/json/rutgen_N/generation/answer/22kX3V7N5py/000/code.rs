// Answer 0

#[test]
fn test_parse_any_signed_number_with_negative() {
    struct MockParser {
        input: Vec<u8>,
        current_index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, current_index: 0 }
        }

        fn peek(&self) -> Option<u8> {
            self.input.get(self.current_index).copied()
        }

        fn eat_char(&mut self) {
            self.current_index += 1;
        }

        fn parse_any_number(&self, is_positive: bool) -> Result<ParserNumber, ErrorCode> {
            // Mock number parsing implementation
            let number = String::from_utf8(self.input[self.current_index..].to_vec()).unwrap();
            Ok(ParserNumber { value: number.parse().unwrap() })
        }

        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err // Simplified for this test
        }

        fn peek_error(&self, err: ErrorCode) -> ErrorCode {
            err // Simplified for this test
        }
    
        fn parse_any_signed_number(&mut self) -> Result<ParserNumber, ErrorCode> {
            // Original function to be tested
            let peek = match self.peek() {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(self.peek_error(ErrorCode::InvalidNumber)),
            };

            let value = match self.peek() {
                Some(_) => Err(self.peek_error(ErrorCode::InvalidNumber)),
                None => value,
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut parser = MockParser::new(b"-123".to_vec());
    let result = parser.parse_any_signed_number();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, -123);
}

#[test]
fn test_parse_any_signed_number_with_positive() {
    struct MockParser {
        input: Vec<u8>,
        current_index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, current_index: 0 }
        }

        fn peek(&self) -> Option<u8> {
            self.input.get(self.current_index).copied()
        }

        fn eat_char(&mut self) {
            self.current_index += 1;
        }

        fn parse_any_number(&self, is_positive: bool) -> Result<ParserNumber, ErrorCode> {
            // Mock number parsing implementation
            let number = String::from_utf8(self.input[self.current_index..].to_vec()).unwrap();
            Ok(ParserNumber { value: number.parse().unwrap() })
        }

        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err // Simplified for this test
        }

        fn peek_error(&self, err: ErrorCode) -> ErrorCode {
            err // Simplified for this test
        }
    
        fn parse_any_signed_number(&mut self) -> Result<ParserNumber, ErrorCode> {
            // Original function to be tested
            let peek = match self.peek() {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(self.peek_error(ErrorCode::InvalidNumber)),
            };

            let value = match self.peek() {
                Some(_) => Err(self.peek_error(ErrorCode::InvalidNumber)),
                None => value,
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut parser = MockParser::new(b"456".to_vec());
    let result = parser.parse_any_signed_number();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 456);
}

#[test]
#[should_panic]
fn test_parse_any_signed_number_should_panic_on_invalid() {
    struct MockParser {
        input: Vec<u8>,
        current_index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, current_index: 0 }
        }

        fn peek(&self) -> Option<u8> {
            self.input.get(self.current_index).copied()
        }

        fn eat_char(&mut self) {
            self.current_index += 1;
        }

        fn parse_any_number(&self, is_positive: bool) -> Result<ParserNumber, ErrorCode> {
            // Mock number parsing implementation
            let number = String::from_utf8(self.input[self.current_index..].to_vec()).unwrap();
            Ok(ParserNumber { value: number.parse().unwrap() })
        }

        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err // Simplified for this test
        }

        fn peek_error(&self, err: ErrorCode) -> ErrorCode {
            err // Simplified for this test
        }
    
        fn parse_any_signed_number(&mut self) -> Result<ParserNumber, ErrorCode> {
            // Original function to be tested
            let peek = match self.peek() {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(self.peek_error(ErrorCode::InvalidNumber)),
            };

            let value = match self.peek() {
                Some(_) => Err(self.peek_error(ErrorCode::InvalidNumber)),
                None => value,
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut parser = MockParser::new(b"abc".to_vec());
    let result = parser.parse_any_signed_number();
    // Expecting panic due to invalid number format
}

