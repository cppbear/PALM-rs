// Answer 0

#[test]
fn test_parse_any_signed_number_negative() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, _is_positive: bool) -> Result<i64, ()> {
            // For testing, assume a simple parse functionality
            let number = self.input[self.index - 1] as char;
            Ok(number.to_digit(10).unwrap() as i64 * if _is_positive { 1 } else { -1 })
        }

        fn peek_error(&self, _: ()) -> () {
            // Dummy implementation for error handling
        }

        fn fix_position(&self, _: ()) -> () {
            // Dummy implementation for fixing position
        }

        fn parse_any_signed_number(&mut self) -> Result<i64, ()> {
            let peek = match self.peek() {
                Ok(Some(b)) => b,
                _ => return Err(()),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(()),
            };

            if self.peek()?.is_ok() {
                return Err(());
            }

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = Parser::new(vec![b'-', b'1']);
    let result = parser.parse_any_signed_number();
    assert_eq!(result, Ok(-1));
}

#[test]
fn test_parse_any_signed_number_positive() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, _is_positive: bool) -> Result<i64, ()> {
            // For testing, assume a simple parse functionality
            let number = self.input[self.index - 1] as char;
            Ok(number.to_digit(10).unwrap() as i64 * if _is_positive { 1 } else { -1 })
        }

        fn peek_error(&self, _: ()) -> () {
            // Dummy implementation for error handling
        }

        fn fix_position(&self, _: ()) -> () {
            // Dummy implementation for fixing position
        }

        fn parse_any_signed_number(&mut self) -> Result<i64, ()> {
            let peek = match self.peek() {
                Ok(Some(b)) => b,
                _ => return Err(()),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(()),
            };

            if self.peek()?.is_ok() {
                return Err(());
            }

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = Parser::new(vec![b'1']);
    let result = parser.parse_any_signed_number();
    assert_eq!(result, Ok(1));
}

#[test]
fn test_parse_any_signed_number_invalid() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, _is_positive: bool) -> Result<i64, ()> {
            // For testing, assume a simple parse functionality
            let number = self.input[self.index - 1] as char;
            Ok(number.to_digit(10).unwrap() as i64 * if _is_positive { 1 } else { -1 })
        }

        fn peek_error(&self, _: ()) -> () {
            // Dummy implementation for error handling
        }

        fn fix_position(&self, _: ()) -> () {
            // Dummy implementation for fixing position
        }

        fn parse_any_signed_number(&mut self) -> Result<i64, ()> {
            let peek = match self.peek() {
                Ok(Some(b)) => b,
                _ => return Err(()),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)
                }
                b'0'..=b'9' => self.parse_any_number(true),
                _ => Err(()),
            };

            if self.peek()?.is_ok() {
                return Err(());
            }

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            }
        }
    }

    let mut parser = Parser::new(vec![b'a']);
    let result = parser.parse_any_signed_number();
    assert!(result.is_err());
}

