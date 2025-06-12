// Answer 0

fn test_ignore_decimal_valid_input() -> Result<()> {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }

    impl Parser {
        fn new(input: Vec<u8>) -> Self {
            Parser { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Invalid number")
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            self.eat_char();
            if let Ok(next) = self.peek_or_null() {
                if next == b'+' || next == b'-' {
                    self.eat_char();
                }
                while let Ok(digit) = self.peek_or_null() {
                    if b'0'..=b'9' == digit {
                        self.eat_char();
                    } else {
                        break;
                    }
                }
            }
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = tri!(self.peek_or_null()) {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(ErrorCode::InvalidNumber));
            }

            match tri!(self.peek_or_null()) {
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = Parser::new(vec![b'1', b'2', b'3', b'e', b'4']);
    assert!(parser.ignore_decimal().is_ok());

    Ok(())
}

#[test]
fn test_ignore_decimal_no_digits() -> Result<()> {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }

    impl Parser {
        fn new(input: Vec<u8>) -> Self {
            Parser { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Invalid number")
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            self.eat_char();
            if let Ok(next) = self.peek_or_null() {
                if next == b'+' || next == b'-' {
                    self.eat_char();
                }
                while let Ok(digit) = self.peek_or_null() {
                    if b'0'..=b'9' == digit {
                        self.eat_char();
                    } else {
                        break;
                    }
                }
            }
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = tri!(self.peek_or_null()) {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(ErrorCode::InvalidNumber));
            }

            match tri!(self.peek_or_null()) {
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = Parser::new(vec![b'.']);
    assert!(parser.ignore_decimal().is_err());

    Ok(())
}

#[test]
fn test_ignore_decimal_with_exponent() -> Result<()> {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }

    impl Parser {
        fn new(input: Vec<u8>) -> Self {
            Parser { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Invalid number")
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            self.eat_char();
            if let Ok(next) = self.peek_or_null() {
                if next == b'+' || next == b'-' {
                    self.eat_char();
                }
                while let Ok(digit) = self.peek_or_null() {
                    if b'0'..=b'9' == digit {
                        self.eat_char();
                    } else {
                        break;
                    }
                }
            }
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = tri!(self.peek_or_null()) {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(ErrorCode::InvalidNumber));
            }

            match tri!(self.peek_or_null()) {
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = Parser::new(vec![b'4', b'2', b'8', b'E', b'5']);
    assert!(parser.ignore_decimal().is_ok());

    Ok(())
}

