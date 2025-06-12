// Answer 0

#[test]
fn test_ignore_integer_valid_1() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // simulating null
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // simulating null
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, code: ErrorCode) -> Result<()> {
            Err(Error::from(code))
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            // Dummy implementation
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            // Dummy implementation
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.error(ErrorCode::InvalidNumber));
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(self.error(ErrorCode::InvalidNumber));
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = TestParser {
        input: b"123.45".to_vec(),
        index: 0,
    };
    assert!(parser.ignore_integer().is_ok());
}

#[test]
fn test_ignore_integer_invalid_leading_zero() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // simulating null
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // simulating null
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, code: ErrorCode) -> Result<()> {
            Err(Error::from(code))
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.error(ErrorCode::InvalidNumber));
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(self.error(ErrorCode::InvalidNumber));
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = TestParser {
        input: b"00.45".to_vec(),
        index: 0,
    };
    assert!(parser.ignore_integer().is_err());
}

#[test]
fn test_ignore_integer_valid_exponent() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // simulating null
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // simulating null
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, code: ErrorCode) -> Result<()> {
            Err(Error::from(code))
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.error(ErrorCode::InvalidNumber));
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(self.error(ErrorCode::InvalidNumber));
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = TestParser {
        input: b"123e10".to_vec(),
        index: 0,
    };
    assert!(parser.ignore_integer().is_ok());
}

