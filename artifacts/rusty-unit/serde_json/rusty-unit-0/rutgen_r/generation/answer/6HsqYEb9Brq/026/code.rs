// Answer 0

fn test_ignore_integer_leading_zero() -> Result<()> {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, index: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = MockParser::new(vec![b'0', b'e']);
    let result = parser.ignore_integer();
    assert!(result.is_ok());
}

fn test_ignore_integer_valid_integer() -> Result<()> {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, index: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = MockParser::new(vec![b'1', b'0', b'0', b'.']);
    let result = parser.ignore_integer();
    assert!(result.is_ok());
}

fn test_ignore_integer_invalid_leading_zero() -> Result<()> {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, index: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = MockParser::new(vec![b'0', b'1']);
    let result = parser.ignore_integer();
    assert!(result.is_err());
}

fn test_ignore_integer_exponent() -> Result<()> {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            MockParser { input, index: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = MockParser::new(vec![b'2', b'e', b'1']);
    let result = parser.ignore_integer();
    assert!(result.is_ok());
}

