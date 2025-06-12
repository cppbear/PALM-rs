// Answer 0

#[test]
fn test_ignore_integer_leading_zero_invalid() {
    struct DummyParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl DummyParser {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0) // Representing null
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0) // Representing null
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            // Dummy implementation
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            // Dummy implementation
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<(), &'static str> {
            Err("Invalid number")
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }
    }

    let mut parser = DummyParser { input: vec![b'0', b'1'], pos: 0 };
    let result = parser.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_valid() {
    struct DummyParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl DummyParser {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0) // Representing null
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0) // Representing null
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<(), &'static str> {
            Err("Invalid number")
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }
    }

    let mut parser = DummyParser { input: vec![b'1', b'2', b'3'], pos: 0 };
    let result = parser.ignore_integer();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_integer_invalid_non_digit() {
    struct DummyParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl DummyParser {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0) // Representing null
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0) // Representing null
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<(), &'static str> {
            Err("Invalid number")
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }
    }

    let mut parser = DummyParser { input: vec![b'2', b'3', b'-'], pos: 0 };
    let result = parser.ignore_integer();
    assert!(result.is_err());
}

