// Answer 0

#[test]
fn test_ignore_integer_invalid_number_leading_zero() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.position += 1;
                Ok(c)
            } else {
                Ok(0) // Simulating end of input with null byte
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Simulating end of input with null byte
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> Result<()> {
            Err(Error::new(ErrorCode::InvalidNumber))
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            // Mock implementation for ignore_decimal
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            // Mock implementation for ignore_exponent
            Ok(())
        }
    }

    let mut parser = MockParser {
        input: vec![b'0', b'1'], // leading zero followed by a digit
        position: 0,
    };

    let result = parser.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_invalid_number_non_digit() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.position += 1;
                Ok(c)
            } else {
                Ok(0) // Simulating end of input with null byte
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Simulating end of input with null byte
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> Result<()> {
            Err(Error::new(ErrorCode::InvalidNumber))
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = MockParser {
        input: vec![b'a'], // non-digit character
        position: 0,
    };

    let result = parser.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_valid_case() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.position += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> Result<()> {
            Err(Error::new(ErrorCode::InvalidNumber))
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = MockParser {
        input: vec![b'1', b'2', b'3'], // valid number
        position: 0,
    };

    let result = parser.ignore_integer();
    assert!(result.is_ok());
}

